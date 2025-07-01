use crate::Messages;
use crate::parser::SbfParser;

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
                            (br, true)
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
    use libsbf::{Messages, reader::SbfReader};
    use std::io::BufRead;

    #[test]
    fn sbf_correct_parse() -> Result<()> {
        let input_stream = std::fs::File::open("test-files/sbf_binary.log")?;
        let correct_file = std::fs::File::open("test-files/correct_sbf_output.log")?;
        let mut cf_lines = std::io::BufReader::new(correct_file).lines();

        let sbf_reader = SbfReader::new(input_stream);

        for m in sbf_reader {
            match m? {
                Messages::INSNavGeod(ins_nav_geod) => {
                    let parsed = format!("{:?}",ins_nav_geod);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(parsed == expected, "parsed line: {} did not match expected line: {}", parsed, expected);
                    
                }
                Messages::AttEuler(att_euler) => {
                    let parsed = format!("{:?}",att_euler);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(parsed == expected, "parsed line: {} did not match expected line: {}", parsed, expected);
                }
                Messages::ExtSensorMeas(ext_sensor_meas) => {
                    let parsed = format!("{:?}",ext_sensor_meas);
                    let expected = cf_lines.next().unwrap()?;
                    assert!(parsed == expected, "parsed line: {} did not match expected line: {}", parsed, expected);
                }
                /// TODO: Update Test to include IMUSetup
                // Messages::ImuSetup(imu_setup) => {
                //     let parsed = format!("{:?}",imu_setup);
                //     let expected = cf_lines.next().unwrap()?;
                //     assert!(parsed == expected, "parsed line: {} did not match expected line: {}", parsed, expected);
                // }
                _ => continue
            }
        }
        Ok(())
    }
}


