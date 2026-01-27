use afl::*;

use libsbf::parser::SbfParser;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut parser = SbfParser::new();

        match parser.consume(data) {
            Some(_msg) => {
                eprintln!("got msg");
            }

            None => {
                eprintln!("got no msg");
            }
        }
    })
}
