use crate::{parser::SbfParser, Messages};
use std::{
    io::{self, ErrorKind, Read},
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

// NOTE: May make this tunable. The std reader is going to be on user
// space linux and in many cases users will have the memory.
// 8K is the default size of the BufReader in rust.
const BUFFER_SIZE: usize = 1024 * 8;
const UDP_BUFFER_SIZE: usize = 65536;

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

    pub fn last_raw_bytes(&self) -> Option<&[u8]> {
        self.parser.last_raw_bytes()
    }
}

impl<R: Read> Iterator for SbfReader<R> {
    type Item = io::Result<Messages>;

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

/// Read SBF data from UDP datagrams, presenting them as a byte stream.
///
/// Each UDP datagram is buffered internally so that [`SbfReader`] can consume
/// it in chunks. Returns EOF (zero bytes) when the read timeout elapses with
/// no data, allowing the caller to detect connection loss.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct UdpReader {
    socket: UdpSocket,
    buf: Vec<u8>,
    pos: usize,
    used: usize,
}

impl UdpReader {
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn try_new(port: u16, timeout: Duration) -> io::Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{port}"))?;
        socket.set_read_timeout(Some(timeout))?;
        Ok(Self {
            socket,
            buf: vec![0u8; UDP_BUFFER_SIZE],
            pos: 0,
            used: 0,
        })
    }
}

impl Read for UdpReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.used {
            loop {
                match self.socket.recv(&mut self.buf) {
                    // Zero-length datagrams are legal but carry no SBF data; skip them.
                    Ok(0) => continue,
                    Ok(n) => {
                        self.used = n;
                        self.pos = 0;
                        break;
                    }
                    Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                        return Ok(0);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        let to_copy = (self.used - self.pos).min(buf.len());
        buf[..to_copy].copy_from_slice(&self.buf[self.pos..self.pos + to_copy]);
        self.pos += to_copy;
        Ok(to_copy)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use libsbf::{
        reader::{SbfReader, UdpReader},
        Messages,
    };
    use std::{
        fs::{read, File},
        io::{self, BufRead, BufReader, Cursor, Read, Write},
        net::UdpSocket,
        sync::{Arc, Barrier},
        time::Duration,
    };

    fn check_parse(
        sbf_reader: impl Iterator<Item = io::Result<Messages>>,
        cf_lines: &mut impl Iterator<Item = io::Result<String>>,
    ) {
        sbf_reader
            .filter_map(|m| match m.expect("sbf parse error") {
                Messages::INSNavGeod(v) => Some(format!("{:?}", v)),
                Messages::AttEuler(v) => Some(format!("{:?}", v)),
                Messages::ExtSensorMeas(v) => Some(format!("{:?}", v)),
                _ => None,
            })
            .for_each(|parsed| {
                let expected = cf_lines
                    .next()
                    .expect("expected output exhausted before messages")
                    .expect("error reading expected output");
                assert!(
                    parsed == expected,
                    "parsed line: {} did not match expected line: {}",
                    parsed,
                    expected
                );
            });
        assert!(cf_lines.next().is_none(), "expected output was not fully consumed");
    }

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
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
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
        let input_stream = File::open("test-files/sbf_binary.log")?;
        let correct_file = File::open("test-files/correct_sbf_output.log")?;
        let mut cf_lines = BufReader::new(correct_file).lines();

        let sbf_reader = SbfReader::new(input_stream);
        // TODO: Update Test to include IMUSetup
        check_parse(sbf_reader, &mut cf_lines);
        Ok(())
    }

    #[test]
    fn test_udp_timeout_returns_eof() {
        let mut reader = UdpReader::try_new(0, Duration::from_millis(100)).unwrap();
        let mut buf = [0u8; 64];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0, "UdpReader should return EOF on timeout");
    }

    #[test]
    fn test_udp_empty_datagram_skipped() {
        let mut reader = UdpReader::try_new(0, Duration::from_millis(200)).unwrap();
        let port = reader.local_addr().unwrap().port();
        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();

        sender.send_to(&[], format!("127.0.0.1:{port}")).unwrap();
        let data: Vec<u8> = (0..64).map(|i| i as u8).collect();
        sender.send_to(&data, format!("127.0.0.1:{port}")).unwrap();

        let mut received = Vec::new();
        let mut buf = [0u8; 1024];
        loop {
            let n = reader.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            received.extend_from_slice(&buf[..n]);
        }

        assert_eq!(received, data);
    }

    #[test]
    fn test_udp_random_data_consumption() {
        let mut reader = UdpReader::try_new(0, Duration::from_millis(200)).unwrap();
        let port = reader.local_addr().unwrap().port();
        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();

        let datagram_sizes = [100usize, 1024, 8192, 16384];
        let mut all_sent: Vec<u8> = Vec::new();
        for &size in &datagram_sizes {
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            sender.send_to(&data, format!("127.0.0.1:{port}")).unwrap();
            all_sent.extend_from_slice(&data);
        }

        let mut received = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            let n = reader.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            received.extend_from_slice(&buf[..n]);
        }

        assert_eq!(received, all_sent);
    }

    #[test]
    fn test_udp_correct_parse() -> Result<()> {
        let mut reader = UdpReader::try_new(0, Duration::from_millis(200)).unwrap();
        let port = reader.local_addr().unwrap().port();

        let correct_file = File::open("test-files/correct_sbf_output.log")?;
        let mut cf_lines = BufReader::new(correct_file).lines();

        // sbf_binary.log (~1.1 MB) exceeds the OS UDP receive buffer (~208 KB).
        // BarrierReader gates each recv: the sender only advances after the
        // receiver has consumed the packet, so the OS buffer never accumulates
        // more than one outstanding datagram.
        struct BarrierReader<'a> {
            inner: &'a mut UdpReader,
            barrier: Arc<Barrier>,
        }
        impl Read for BarrierReader<'_> {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                let n = self.inner.read(buf)?;
                // Signal the sender that this packet has been consumed so it
                // can send the next one without overflowing the OS buffer.
                if n > 0 {
                    self.barrier.wait();
                }
                Ok(n)
            }
        }

        let data = read("test-files/sbf_binary.log")?;
        let barrier = Arc::new(Barrier::new(2));
        let sender_barrier = barrier.clone();
        std::thread::spawn(move || {
            let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
            for chunk in data.chunks(1024) {
                sender.send_to(chunk, format!("127.0.0.1:{port}")).unwrap();
                sender_barrier.wait();
            }
        });

        let sbf_reader = SbfReader::new(BarrierReader { inner: &mut reader, barrier });
        check_parse(sbf_reader, &mut cf_lines);
        Ok(())
    }

    #[test]
    fn raw_bytes_in_reader() -> Result<()> {
        const SYNC: &[u8] = &[36, 64];
        const QUALITY_IND_HEADER: &[u8] = &[134, 98, 242, 15, 32, 0];
        const QUALITY_IND_PAYLOAD: &[u8] = &[
            184, 244, 58, 29, 56, 9, 7, 0, 11, 10, 12, 10, 1, 0, 2, 0, 21, 10, 31, 0, 0, 0, 0, 0,
        ];

        let mut input = Vec::new();
        for _ in 0..3 {
            input.extend_from_slice(SYNC);
            input.extend_from_slice(QUALITY_IND_HEADER);
            input.extend_from_slice(QUALITY_IND_PAYLOAD);
        }

        let mut recording = Cursor::new(Vec::new());
        let mut reader = SbfReader::new(input.as_slice());
        while let Some(result) = reader.next() {
            result?;
            if let Some(raw) = reader.last_raw_bytes() {
                recording.write_all(raw)?;
            }
        }

        assert_eq!(recording.into_inner(), input);
        Ok(())
    }
}
