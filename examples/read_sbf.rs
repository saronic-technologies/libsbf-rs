use libsbf::reader::SbfReader;

use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::net::TcpStream;

use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source: file path or TCP address (host:port)
    ///
    /// Examples:
    ///   - /path/to/file.sbf (read from file)
    ///   - 192.168.1.100:5555 (connect to TCP)
    ///   - 127.0.0.1:8080 (default if not specified)
    #[arg(default_value = "127.0.0.1:8080")]
    input: String,

    /// Print message details (not just statistics)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();

    // Check if argument looks like a file path or TCP address
    let reader: Box<dyn Read> = if args.input.contains(':') {
        eprintln!("Connecting to TCP: {}", args.input);
        Box::new(TcpStream::connect(args.input)?)
    } else {
        eprintln!("Reading from file: {}", args.input);
        Box::new(File::open(args.input)?)
    };

    let sbf_reader = SbfReader::new(reader);
    let mut stats: HashMap<String, usize> = HashMap::new();

    for m in sbf_reader {
        let msg = m?;
        if args.verbose {
            println!("{msg:?}");
        }
        let debug = format!("{msg:?}");
        let msg_type = debug.split('(').next().unwrap().to_string();
        *stats.entry(msg_type).or_insert(0) += 1;
    }

    eprintln!("\n=== Message Statistics ===");
    let total: usize = stats.values().sum();
    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (msg_type, count) in &sorted {
        eprintln!("{msg_type}: {count}");
    }
    eprintln!("Total messages: {total}");

    Ok(())
}
