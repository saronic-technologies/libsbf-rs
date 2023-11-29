use anyhow::Context;
use libsbf::{Header, INSNavGeod, Messages};

use binrw::BinRead;
use binrw::io::Cursor;

use std::env;
use std::io::Read;
use std::net::TcpStream;



fn main() -> anyhow::Result<()> {
    let ip_port = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".into());
    eprintln!("{ip_port}");
    let mut stream = TcpStream::connect(ip_port)?;
    loop {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf)?;
        if libsbf::is_sync(&buf) {
            let mut header_buf = [0u8; 6];
            stream.read_exact(&mut header_buf)?;
            let h = Header::read_ne(&mut Cursor::new(header_buf))?;
            let mut body_buf = Vec::<u8>::with_capacity((h.length - 8) as usize);
            stream.read_exact(body_buf.as_mut_slice())?;
            // TODO: do CRC check
            match h.block_id.message_type() {
                Messages::INSNavGeod => {
                    let mut body_cursor = Cursor::new(body_buf.as_slice());
                    let ins_nav_geod = INSNavGeod::read_ne(&mut body_cursor)?;
                    eprintln!("{:?}", ins_nav_geod);
                }
                _ => {
                    eprintln!("Unsupported message");
                }
            }
        }
    }

}
