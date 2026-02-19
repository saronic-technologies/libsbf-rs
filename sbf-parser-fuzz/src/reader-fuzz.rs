use afl::*;
use std::io::Read;

use libsbf::reader::SbfReader;

fn main() {
    fuzz!(|data: &[u8]| {
        // Test with the original fuzz data
        test_sbf_reader(data);
        
        // Also test with larger data to catch buffer-related bugs
        if (0..20000).contains(&data.len()) {
            // Create a larger test case by repeating the data
            let mut large_data = Vec::new();
            while large_data.len() < 20000 {
                large_data.extend_from_slice(data);
            }
            test_sbf_reader(&large_data);
        }
    });
}

fn test_sbf_reader(data: &[u8]) {
    // Create a simple reader from the fuzz data
    let mut reader = FuzzReader::new(data);
    let total_bytes = reader.len();
    
    // Skip empty data
    if total_bytes == 0 {
        return;
    }
    
    // Process all messages from the SbfReader
    let sbf_reader = SbfReader::try_new(&mut reader, "test").unwrap();
    let mut message_count = 0;
    
    for result in sbf_reader {
        match result {
            Ok(_msg) => {
                message_count += 1;
            }
            Err(_) => {
                // Errors are expected with random data, but shouldn't panic
            }
        }
    }
    
    // The reader should have consumed all the data
    let bytes_consumed = reader.bytes_read();
    assert_eq!(bytes_consumed, total_bytes, 
        "SbfReader did not consume all {} bytes of fuzz data, only consumed {}", 
        total_bytes, bytes_consumed);
}

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
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn bytes_read(&self) -> usize {
        self.position
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
