use libsbf::reader::SbfReader;

use std::env;
use std::net::TcpStream;

use tracing_subscriber::EnvFilter;


fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let ip_port = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into());
    eprintln!("{ip_port}");
    let stream = TcpStream::connect(ip_port)?;
    let sbf_reader = SbfReader::new(stream);

    for m in sbf_reader {
        eprintln!("{:?}", m);
    }
    Ok(())
}
