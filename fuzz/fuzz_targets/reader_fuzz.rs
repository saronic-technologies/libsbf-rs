#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Read;

use libsbf::reader::SbfReader;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let mut reader = FuzzReader::new(data);
    let sbf_reader = SbfReader::new(&mut reader);

    // Just iterate through all messages without panicking
    for _result in sbf_reader {
        // Errors are expected with random data
    }
});

// A simple reader that provides all fuzz data at once
struct FuzzReader {
    data: Vec<u8>,
    position: usize,
}

impl FuzzReader {
    fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            position: 0,
        }
    }
    
    fn _len(&self) -> usize {
        self.data.len()
    }
}

impl Read for FuzzReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.position;
        let to_read = buf.len().min(remaining);
        
        if to_read > 0 {
            buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
        }
        
        Ok(to_read)
    }
}
