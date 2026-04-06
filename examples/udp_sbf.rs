use libsbf::parse_datagram;

use clap::Parser;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(about = "Listen for SBF datagrams on UDP and print message counts")]
struct Args {
    /// UDP port to listen on
    #[arg(default_value_t = 28785)]
    port: u16,

    /// Print interval in seconds
    #[arg(short, long, default_value_t = 5)]
    interval: u64,

    /// Print full message debug output
    #[arg(short, long)]
    verbose: bool,
}

/// Parse all SBF blocks from a buffer. The device may pack multiple blocks
/// into one UDP datagram.
fn parse_all_blocks(data: &[u8]) -> Vec<Result<libsbf::Messages, libsbf::DatagramError>> {
    let mut results = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        let remaining = &data[offset..];
        let sync_pos = remaining.windows(2).position(|w| w == b"$@");

        let start = match sync_pos {
            Some(pos) => offset + pos,
            None => break,
        };

        match parse_datagram(&data[start..]) {
            Ok(msg) => {
                // Read block length from header to advance past this block
                if data.len() >= start + 8 {
                    let length =
                        u16::from_le_bytes([data[start + 6], data[start + 7]]) as usize;
                    offset = start + length;
                } else {
                    offset = data.len();
                }
                results.push(Ok(msg));
            }
            Err(e) => {
                results.push(Err(e));
                offset = start + 2;
            }
        }
    }

    results
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", args.port))?;
    eprintln!("Listening on UDP port {}", args.port);

    let mut stats: HashMap<String, usize> = HashMap::new();
    let mut last_print = Instant::now();
    let mut buf = [0u8; 65536];
    let mut datagrams: usize = 0;
    let mut multi_block: usize = 0;

    loop {
        let n = socket.recv(&mut buf)?;
        datagrams += 1;

        let blocks = parse_all_blocks(&buf[..n]);
        if blocks.len() > 1 {
            multi_block += 1;
        }

        for result in blocks {
            match result {
                Ok(msg) => {
                    if args.verbose {
                        println!("{msg:?}");
                    }
                    let debug = format!("{msg:?}");
                    let msg_type = debug.split('(').next().unwrap().to_string();
                    *stats.entry(msg_type).or_insert(0) += 1;
                }
                Err(e) => {
                    *stats.entry(format!("Error({e:?})")).or_insert(0) += 1;
                }
            }
        }

        if last_print.elapsed().as_secs() >= args.interval {
            let total: usize = stats.values().sum();
            let mut sorted: Vec<_> = stats.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            eprintln!("--- {total} msgs from {datagrams} datagrams ({multi_block} multi-block) ---");
            for (msg_type, count) in &sorted {
                eprintln!("  {msg_type}: {count}");
            }
            stats.clear();
            datagrams = 0;
            multi_block = 0;
            last_print = Instant::now();
        }
    }
}
