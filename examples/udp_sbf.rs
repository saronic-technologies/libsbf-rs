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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", args.port))?;
    eprintln!("Listening on UDP port {}", args.port);

    let mut stats: HashMap<String, usize> = HashMap::new();
    let mut last_print = Instant::now();
    let mut buf = [0u8; 65536];

    loop {
        let n = socket.recv(&mut buf)?;

        match parse_datagram(&buf[..n]) {
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

        if last_print.elapsed().as_secs() >= args.interval {
            let total: usize = stats.values().sum();
            let mut sorted: Vec<_> = stats.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            eprintln!("--- {total} messages ---");
            for (msg_type, count) in &sorted {
                eprintln!("  {msg_type}: {count}");
            }
            stats.clear();
            last_print = Instant::now();
        }
    }
}
