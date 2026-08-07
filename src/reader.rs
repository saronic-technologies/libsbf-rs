use crate::parser::SbfParser;
use crate::Messages;

use std::io::Read;

// NOTE: May make this tunable. The std reader is going to be on user
// space linux and in many cases users will have the memory.
// 8K is the default size of the BufReader in rust.
const BUFFER_SIZE: usize = 1024 * 8;

/// Read SBF data via a BuffReader and Iterator.
///
/// # Examples
///
/// ```no_run
/// use libsbf::reader::SbfReader;
/// use std::env;
/// use std::net::TcpStream;
///
/// fn main() -> anyhow::Result<()> {
///     let stream = TcpStream::connect("127.0.0.1:8080")?;
///     let sbf_reader = SbfReader::new(stream);
///     for m in sbf_reader {
///         eprintln!("{:?}", m);
///     }
///     Ok(())
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct SbfReader<R: Read> {
    reader: R,
    parser: SbfParser,
    drain_internal: bool,
}

impl<R: Read> SbfReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            parser: SbfParser::new(),
            drain_internal: false,
        }
    }

    /// Raw bytes of the most recently parsed message, or `None` before the
    /// first successful parse. Bytes skipped while scanning for a valid
    /// message are not captured.
    ///
    /// A `for` loop mutably borrows the reader for the entire loop body,
    /// so this method can not be used directly. A `while` loop only
    /// borrows the iterator for the call to `next()`, so this method can
    /// be used in the body of the loop. For example:
    ///
    /// ```no_run
    /// use libsbf::reader::SbfReader;
    /// use std::net::TcpStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let stream = TcpStream::connect("127.0.0.1:8080")?;
    ///     let mut reader = SbfReader::new(stream);
    ///     let mut recording = Vec::new();
    ///     while let Some(message) = reader.next() {
    ///         let message = message?;
    ///         if let Some(raw) = reader.last_raw_bytes() {
    ///             recording.extend_from_slice(raw);
    ///         }
    ///         eprintln!("{:?}", message);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn last_raw_bytes(&self) -> Option<&[u8]> {
        self.parser.last_raw_bytes()
    }
}

impl<R: Read> Iterator for SbfReader<R> {
    type Item = Result<Messages, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            tracing::debug!("Trying to read from reader");
            let (bytes_read, is_eof) = {
                if self.drain_internal {
                    (0, false)
                } else {
                    match self.reader.read(&mut buffer) {
                        Ok(br) => {
                            tracing::debug!("Successfully read {br} bytes from reader");
                            (br, br == 0)
                        }
                        Err(e) => {
                            return Some(Err(e));
                        }
                    }
                }
            };

            match self.parser.consume(&buffer[..bytes_read]) {
                Some(msg) => {
                    // NOTE: When we get a message the parser still
                    // contains the internal buffer so lets drain that
                    // all the way down until we get a None which
                    // indicates that the parser needs more data to
                    // get messages. Instead of constantly growing
                    // that buffer by reading more data from the
                    // reader we first want to have it go down to
                    // reduce memory usage and work for the internal
                    // parser.
                    self.drain_internal = true;
                    return Some(Ok(msg));
                }
                None => {
                    self.drain_internal = false;
                    // loop
                }
            }

            if is_eof {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use libsbf::{reader::SbfReader, Messages};
    use std::io::{BufRead, Read};

    #[test]
    fn test_random_data_consumption() {
        // Create a reader that tracks how many bytes were read
        struct TrackingReader {
            data: Vec<u8>,
            position: usize,
        }

        impl TrackingReader {
            fn new(size: usize) -> Self {
                // Generate random data
                let data: Vec<_> = (0..size).map(|i| (i % 256) as u8).collect();
                Self { data, position: 0 }
            }

            fn bytes_read(&self) -> usize {
                self.position
            }

            fn total_bytes(&self) -> usize {
                self.data.len()
            }
        }

        impl Read for TrackingReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                let remaining = self.data.len() - self.position;
                let to_read = buf.len().min(remaining);

                if to_read > 0 {
                    buf[..to_read]
                        .copy_from_slice(&self.data[self.position..self.position + to_read]);
                    self.position += to_read;
                }

                Ok(to_read)
            }
        }

        // Test with various data sizes
        let test_sizes = vec![100, 1024, 8192, 16384, 100000];

        for size in test_sizes {
            let mut reader = TrackingReader::new(size);
            let total_bytes = reader.total_bytes();

            let sbf_reader = SbfReader::new(&mut reader);

            // Consume all messages (valid or invalid)
            let mut message_count = 0;
            let mut error_count = 0;

            for result in sbf_reader {
                match result {
                    Ok(_) => message_count += 1,
                    Err(_) => error_count += 1,
                }
            }

            // Verify that all bytes were consumed
            assert_eq!(
                reader.bytes_read(),
                total_bytes,
                "SbfReader did not consume all bytes. Read {} out of {} bytes",
                reader.bytes_read(),
                total_bytes
            );

            println!(
                "Test passed for {} bytes: {} messages parsed, {} errors",
                size, message_count, error_count
            );
        }
    }

    #[test]
    fn sbf_correct_parse() -> Result<()> {
        let input_stream = std::fs::File::open("test-files/sbf_binary.log")?;
        let correct_file = std::fs::File::open("test-files/correct_sbf_output.log")?;
        let mut cf_lines = std::io::BufReader::new(correct_file).lines();

        let sbf_reader = SbfReader::new(input_stream);

        for m in sbf_reader {
            match m? {
                Messages::INSNavGeod(ins_nav_geod) => {
                    let parsed = format!("{:?}", ins_nav_geod);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(
                        parsed == expected,
                        "parsed line: {} did not match expected line: {}",
                        parsed,
                        expected
                    );
                }
                Messages::AttEuler(att_euler) => {
                    let parsed = format!("{:?}", att_euler);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(
                        parsed == expected,
                        "parsed line: {} did not match expected line: {}",
                        parsed,
                        expected
                    );
                }
                Messages::ExtSensorMeas(ext_sensor_meas) => {
                    let parsed = format!("{:?}", ext_sensor_meas);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(
                        parsed == expected,
                        "parsed line: {} did not match expected line: {}",
                        parsed,
                        expected
                    );
                }
                // TODO: Update Test to include IMUSetup
                // Messages::ImuSetup(imu_setup) => {
                //     let parsed = format!("{:?}",imu_setup);
                //     let expected = cf_lines.next().unwrap()?;
                //     assert!(parsed == expected, "parsed line: {} did not match expected line: {}", parsed, expected);
                // }
                _ => continue,
            }
        }
        Ok(())
    }

    #[test]
    fn test_raw_bytes_in_reader() {
        const SYNC: &[u8] = &[36, 64];
        const QUALITY_IND_HEADER: &[u8] = &[134, 98, 242, 15, 32, 0];
        const QUALITY_IND_PAYLOAD: &[u8] = &[
            184, 244, 58, 29, 56, 9, 7, 0, 11, 10, 12, 10, 1, 0, 2, 0, 21, 10, 31, 0, 0, 0, 0, 0,
        ];

        let mut frame = Vec::new();
        frame.extend_from_slice(SYNC);
        frame.extend_from_slice(QUALITY_IND_HEADER);
        frame.extend_from_slice(QUALITY_IND_PAYLOAD);

        // Garbage runs of one, two, and three bytes separate the frames.
        // None of the garbage bytes form a "$@" sync sequence, so the scan
        // skips them and locks onto each real frame.
        let input = [
            frame.as_slice(),
            &[0x11],
            frame.as_slice(),
            &[0x22, 0x33],
            frame.as_slice(),
            &[0x44, 0x55, 0x66],
            frame.as_slice(),
        ]
        .concat();

        let mut reader = SbfReader::new(input.as_slice());
        let mut captured = Vec::new();
        while let Some(result) = reader.next() {
            result.unwrap();
            let raw = reader.last_raw_bytes().expect("raw bytes after a parsed message");
            captured.push(raw.to_vec());
        }

        // Each capture is exactly its frame's raw bytes; the garbage
        // separators are skipped, not recorded.
        assert_eq!(captured, vec![frame; 4]);
    }
}
