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
}

impl<R: Read> SbfReader<R> {
    pub fn new(reader: R) -> Self {
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
            tracing::debug!("Trying to read from reader");
            let bytes_read = match self.reader.read(&mut buffer) {
                Ok(br) => {
                    tracing::debug!("Successfully read {br} bytes from reader");
                    br
                }
                Err(e) => {
                    return Some(Err(e));
                }
            };

            if bytes_read == 0 {
                return None;
            }

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

