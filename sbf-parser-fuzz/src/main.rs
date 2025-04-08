use afl::*;

use libsbf::parser::SbfParser;

fn main() {
    fuzz!(|data: &[u8]|{
        let mut parser = SbfParser::new();
        let mut remaining = data;
        
        while !remaining.is_empty() {
            match parser.consume(remaining) {
                Ok((Some(_message), bytes_consumed)) => {
                    eprintln!("got msg");
                    remaining = &remaining[bytes_consumed..];
                },
                Ok((None, bytes_consumed)) => {
                    eprintln!("got None");
                    remaining = &remaining[bytes_consumed..];
                },
                Err(_) => {
                    // Any error from the parser is healthy. We should have no other panics.
                    break;
                 }
            }
        }
    })
}

