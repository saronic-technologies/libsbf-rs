use crate::Messages;
use crate::parser::SbfParser;

use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 4096;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(crate::parser::Error),
}

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
    reader: BufReader<R>,
    parser: SbfParser,
    buffer: heapless::Vec<u8, BUFFER_SIZE>,
}

impl<R: Read> SbfReader<R> {
    pub fn new(r: R) -> Self {
        let reader = BufReader::new(r);
        Self {
            reader,
            parser: SbfParser::new(),
            buffer: heapless::Vec::new(),
        }
    }
}

impl<R: Read> Iterator for SbfReader<R> {
    type Item = Result<Messages, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.buffer.is_empty() {
            match self.parser.consume(&self.buffer) {
                Ok((res, bytes_consumed)) => {
                    assert!(bytes_consumed > 0, "parser consumed 0 bytes, this is impossible!"); // parser.consumed can never return 0 bytes consumed

                    let prev_b = self.buffer.clone();
                    self.buffer.clear();
                    let _ = self.buffer.extend_from_slice(&prev_b[bytes_consumed..]);

                    if let Some(msg) = res {
                        return Some(Ok(msg));
                    }
                },
                Err(e) => return Some(Err(Error::ParseError(e))),
            }
        }

        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            let bytes_read = match self.reader.read(&mut buffer) {
                Ok(b) => b,
                Err(e) => return Some(Err(Error::IOError(e))),
            };
            if bytes_read == 0 {
                return None;
            }

            match self.parser.consume(&buffer[..bytes_read]) {
                Ok((res, bytes_consumed)) => {
                    assert!(bytes_consumed > 0, "parser consumed 0 bytes, this is impossible!"); // parser.consumed can never return 0 bytes consumed
                    if let Some(msg) = res {
                        let _ = self.buffer.extend_from_slice(&buffer[bytes_consumed..]);
                        return Some(Ok(msg));
                    }
                },
                Err(e) => return Some(Err(Error::ParseError(e))),
            }
        }
    }
}

