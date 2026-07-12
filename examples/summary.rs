use anyhow::Result;
use clap::Parser;
use libsbf::{
    reader::{SbfReader, UdpReader},
    Messages,
};
use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{self, Read},
    net::TcpStream,
    time::{Duration, Instant},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Summarize SBF message counts and rates
#[derive(Parser, Debug)]
struct Args {
    /// Input source: file path, TCP address (`host:port`), or UDP port number
    ///
    /// Examples:
    ///
    ///   - `/path/to/file.sbf` — read from file, print final summary
    ///
    ///   - `192.168.1.100:5555` — connect via TCP, print summary each interval
    ///
    ///   - `5555` — listen on UDP port, print summary each interval
    input: String,

    /// Summary interval in seconds; ignored in verbose mode (TCP/UDP only)
    #[arg(short, long, default_value_t = 10.0)]
    interval: f64,

    /// UDP read timeout in seconds; defaults to half the interval (UDP only)
    #[arg(short, long)]
    timeout: Option<f64>,

    /// Print full message debug output; suppresses interval summaries
    #[arg(short, long)]
    verbose: bool,
}

struct MessageStats {
    count: u64,
    first_tow: Option<u32>,
    last_tow: Option<u32>,
}

impl MessageStats {
    fn new(msg: &Messages) -> Self {
        let tow = msg.tow();
        Self {
            count: 1,
            first_tow: tow,
            last_tow: tow,
        }
    }

    fn update(&mut self, msg: &Messages) {
        self.count += 1;
        if let Some(tow) = msg.tow() {
            self.last_tow = Some(tow);
        }
    }
}

impl Display for MessageStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.count)?;
        if self.count >= 2 {
            if let (Some(first), Some(last)) = (self.first_tow, self.last_tow) {
                if last > first {
                    let t = (last - first) as f64 / 1000.0;
                    let rate = (self.count - 1) as f64 / t;
                    let uncertainty = 1.0 / t;
                    write!(f, " ({rate:.2} ± {uncertainty:.2} Hz)")?;
                }
            }
        }
        Ok(())
    }
}

fn print_summary(stats: &HashMap<&'static str, MessageStats>, elapsed_secs: f64) {
    let total: u64 = stats.values().map(|s| s.count).sum();
    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by_key(|b| Reverse(b.1.count));
    info!("Stats for the last {elapsed_secs:.1}s:");
    info!("  Total: {total}");
    for (msg_type, s) in sorted {
        info!("  {msg_type}: {s}");
    }
}

fn run(reader: SbfReader<impl Read>, interval: Option<f64>, verbose: bool) -> Result<()> {
    let mut stats: HashMap<&'static str, MessageStats> = HashMap::new();
    let mut window_start = Instant::now();

    for result in reader {
        let msg = result?;
        if verbose {
            println!("{msg:?}");
        }
        stats
            .entry(msg.type_name())
            .and_modify(|s| s.update(&msg))
            .or_insert_with(|| MessageStats::new(&msg));

        if !verbose {
            let elapsed = window_start.elapsed().as_secs_f64();
            if interval.is_some_and(|i| elapsed >= i) {
                print_summary(&stats, elapsed);
                stats.clear();
                window_start = Instant::now();
            }
        }
    }

    if !stats.is_empty() {
        print_summary(&stats, window_start.elapsed().as_secs_f64());
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .init();

    let args = Args::parse();

    let interval = (!args.verbose).then_some(args.interval);

    // Pure digits → UDP, host:port (no slash before colon) → TCP, otherwise file
    if args.input.chars().all(|c| c.is_ascii_digit()) {
        let port: u16 = args.input.parse()?;
        let timeout = args.timeout.unwrap_or(args.interval / 2.0);
        info!("Listening on UDP port {port} (timeout {timeout:.1}s)");
        run(
            SbfReader::new(UdpReader::try_new(port, Duration::from_secs_f64(timeout))?),
            interval,
            args.verbose,
        )
    } else {
        let (reader, interval): (Box<dyn Read>, Option<f64>) = if args
            .input
            .find(':')
            .is_some_and(|c| !args.input[..c].contains('/'))
        {
            info!("Connecting to TCP: {}", args.input);
            (Box::new(TcpStream::connect(&args.input)?), interval)
        } else {
            info!("Reading from file: {}", args.input);
            (Box::new(File::open(&args.input)?), None)
        };
        run(SbfReader::new(reader), interval, args.verbose)
    }
}
