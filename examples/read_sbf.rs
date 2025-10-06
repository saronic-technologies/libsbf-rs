use libsbf::{Messages, reader::SbfReader};

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
    let mut stats: HashMap<&str, usize> = HashMap::new();
    let mut unsupported_blocks: HashMap<u16, usize> = HashMap::new();
    let mut att_cov_errors: HashMap<u8, usize> = HashMap::new();

    for m in sbf_reader {
        match m? {
            Messages::MeasExtra(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("MeasExtra").or_insert(0) += 1;
            }
            Messages::GALNav(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GALNav").or_insert(0) += 1;
            }
            Messages::MeasEpoch(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("MeasEpoch").or_insert(0) += 1;
            }
            Messages::GALIon(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GALIon").or_insert(0) += 1;
            }
            Messages::GALUtc(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GALUtc").or_insert(0) += 1;
            }
            Messages::GALGstGps(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GALGstGps").or_insert(0) += 1;
            }
            Messages::Meas3Ranges(msg) => {
                if args.verbose {
                    println!("Meas3Ranges: TOW={:?}, raw_bytes={}", msg.tow, msg.raw_data.len());
                }
                *stats.entry("Meas3Ranges").or_insert(0) += 1;
            }
            Messages::Meas3Doppler(msg) => {
                if args.verbose {
                    println!("Meas3Doppler: TOW={:?}, raw_bytes={}", msg.tow, msg.raw_data.len());
                }
                *stats.entry("Meas3Doppler").or_insert(0) += 1;
            }
            Messages::INSSupport(msg) => {
                if args.verbose {
                    println!("INSSupport: TOW={:?}, raw_bytes={}", msg.tow, msg.raw_data.len());
                }
                *stats.entry("INSSupport").or_insert(0) += 1;
            }
            Messages::INSNavGeod(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("INSNavGeod").or_insert(0) += 1;
            }
            Messages::VelSensorSetup(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("VelSensorSetup").or_insert(0) += 1;
            }
            Messages::AttEuler(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("AttEuler").or_insert(0) += 1;
            }
            Messages::AttCovEuler(msg) => {
                if args.verbose {
                    println!("{:#?}", msg);
                }
                *stats.entry("AttCovEuler").or_insert(0) += 1;
                *att_cov_errors.entry(msg.error).or_insert(0) += 1;
            }
            Messages::DiffCorrIn(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("DiffCorrIn").or_insert(0) += 1;
            }
            Messages::ExtSensorMeas(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("ExtSensorMeas").or_insert(0) += 1;
            }
            Messages::ExtSensorStatus(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("ExtSensorStatus").or_insert(0) += 1;
            }
            Messages::ExtSensorInfo(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("ExtSensorInfo").or_insert(0) += 1;
            }
            Messages::ReceiverSetup(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("ReceiverSetup").or_insert(0) += 1;
            }
            Messages::ImuSetup(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("ImuSetup").or_insert(0) += 1;
            }
            Messages::QualityInd(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("QualityInd").or_insert(0) += 1;
            }
            Messages::GEORawL1(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GEORawL1").or_insert(0) += 1;
            }
            Messages::GEONav(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GEONav").or_insert(0) += 1;
            }
            Messages::GPSIon(msg) => {
                if args.verbose {
                    println!("{:?}", msg);
                }
                *stats.entry("GPSIon").or_insert(0) += 1;
            }
            Messages::PosCovGeodetic(msg) => {
                if args.verbose {
                    println!("{:#?}", msg);
                }
                *stats.entry("PosCovGeodetic").or_insert(0) += 1;
            }
            Messages::PVTGeodetic(msg) => {
                if args.verbose {
                    println!("{:#?}", msg);
                }
                *stats.entry("PVTGeodetic").or_insert(0) += 1;
            }
            Messages::ReceiverStatus(msg) => {
                if args.verbose {
                    println!("{:#?}", msg);
                }
                *stats.entry("ReceiverStatus").or_insert(0) += 1;
            }
            Messages::GPSNav(msg) => {
                if args.verbose {
                    println!("{:#?}", msg);
                }
                *stats.entry("GPSNav").or_insert(0) += 1;
            }
            Messages::Unsupported(block_id) => {
                *stats.entry("Unsupported").or_insert(0) += 1;
                *unsupported_blocks.entry(block_id).or_insert(0) += 1;
            }
        }
    }
    
    // Print statistics
    eprintln!("\n=== Message Statistics ===");
    let total: usize = stats.values().sum();
    for (msg_type, count) in stats.iter() {
        eprintln!("{}: {}", msg_type, count);
    }
    eprintln!("Total messages: {}", total);
    
    // Print unsupported blocks
    if !unsupported_blocks.is_empty() {
        eprintln!("\n=== Unsupported Block IDs ===");
        let mut sorted: Vec<_> = unsupported_blocks.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (block_id, count) in sorted.iter().take(10) {
            eprintln!("  0x{:04X} ({}): {} occurrences", block_id, block_id, count);
        }
    }
    
    // Print AttCovEuler error distribution
    if !att_cov_errors.is_empty() {
        eprintln!("\n=== AttCovEuler Error Distribution ===");
        for (error_code, count) in att_cov_errors.iter() {
            eprintln!("  Error {}: {} occurrences", error_code, count);
        }
    }
    
    Ok(())
}
