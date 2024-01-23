use libsbf::{Header, AttEuler, ExtSensorMeas, INSNavGeod, Messages};

use binrw::io::Cursor;
use binrw::BinRead;
use crc16::*;

use std::env;
use std::io::Read;
use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    let ip_port = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into());
    eprintln!("{ip_port}");
    let mut stream = TcpStream::connect(ip_port)?;
    loop {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf)?;
        if libsbf::is_sync(&buf) {
            let mut header_buf = [0u8; 6];
            stream.read_exact(&mut header_buf)?;
            let h = Header::read_le(&mut Cursor::new(header_buf))?;
            if h.length % 4 != 0 || h.length <= 8 {
                continue;
            }
            let mut body_buf = Vec::<u8>::with_capacity((h.length - 8) as usize);
            for _ in 0..(h.length - 8) {
                body_buf.push(0);
            }
            stream.read_exact(&mut body_buf.as_mut_slice())?;
            let mut bb = body_buf.clone();
            let mut full_block = header_buf[2..].to_vec();
            full_block.append(&mut bb);
            let crc = State::<XMODEM>::calculate(full_block.as_slice());
            if h.crc != crc {
                eprintln!("bad crc, expected {:?} and got {crc}", h.crc);
                continue;
            }
            match h.block_id.message_type() {
                Messages::INSNavGeod => {
                    let mut body_cursor = Cursor::new(body_buf.as_slice());
                    let ins_nav_geod = INSNavGeod::read_le(&mut body_cursor)?;
                    eprintln!("{:?}", ins_nav_geod);
                },
                Messages::AttEuler => {
                    let mut body_cursor = Cursor::new(body_buf.as_slice());
                    let att_euler = AttEuler::read_le(&mut body_cursor)?;
                    eprintln!("{:?}", att_euler);
                }
                Messages::ExtSensorMeas => {
                    let mut body_cursor = Cursor::new(body_buf.as_slice());
                    let ext_sensor_meas = ExtSensorMeas::read_le(&mut body_cursor)?;
                    eprintln!("{:?}", ext_sensor_meas);
                }
                _ => continue,
            }
        }
    }
}
