use crate::Messages;
use crate::parser::SbfParser;

use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 4096;

// #[derive(Debug)]
// pub enum Error {
//     IOError(std::io::Error),
//     ParseError(crate::parser::Error),
// }

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
}

impl<R: Read> SbfReader<R> {
    pub fn new(r: R) -> Self {
        let reader = BufReader::new(r);
        Self {
            reader,
            parser: SbfParser::new(),
        }
    }
}

impl<R: Read> Iterator for SbfReader<R> {
    type Item = Result<Messages, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            let bytes_read = match self.reader.read(&mut buffer) {
                Ok(br) => {
                    br
                }
                Err(e) => {
                    return Some(Err(e));
                }
            };

            match self.parser.consume(&buffer[..bytes_read]) {
                Some(msg) => {
                    return Some(Ok(msg));
                }
                None => {
                    // loop
                }
            }
        }
    }
}

