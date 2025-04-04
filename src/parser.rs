extern crate alloc;
use alloc::vec::Vec;

use binrw::io::Cursor;
use binrw::BinRead;

use crate::{Header, Messages, AttEuler, INSNavGeod, ExtSensorMeas, QualityInd};

use crc16::*;


#[derive(Debug)]
pub enum Error {
    InvalidHeaderCRC,
    BinRWError(binrw::Error),
}

enum ParserState {
    Sync1,
    Sync2,
    Header {
        header: heapless::Vec<u8, 6>, // statically allocated
    },
    Payload {
        header: Header,
        header_buf: Vec<u8>,
        payload: Vec<u8>, // dynamically allocated, dependent on length in Header
    },
}

pub struct SbfParser {
    state: ParserState,
}

impl SbfParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Sync1,
        }
    }

    /// Consume an input slice. Returns Some(Message) when a full message is parsed.
    pub fn consume(&mut self, input: &[u8]) -> Result<(Option<Messages>, usize), Error> {
        for (i, &byte) in input.iter().enumerate() {
            if let Some(msg) = self.process_byte(byte)? {
                return Ok((Some(msg), i + 1));
            }
        }
        Ok((None, input.len()))
    }

    /// Processes one byte at a time.
    fn process_byte(&mut self, byte: u8) -> Result<Option<Messages>, Error> {
        match &mut self.state {
            ParserState::Sync1 => {
                // Look for the '$' character.
                if byte == b'$' {
                    self.state = ParserState::Sync2;
                }
                Ok(None)
            }
            ParserState::Sync2 => {
                // Now expect the '@' to complete the sync sequence.
                if byte == b'@' {
                    self.state = ParserState::Header {
                        header: heapless::Vec::<u8, 6>::new(),
                    };
                } else if byte == b'$' {
                    // If another '$' is seen, remain in Sync2.
                    self.state = ParserState::Sync2;
                } else {
                    // Any other character resets to Sync1.
                    self.state = ParserState::Sync1;
                }
                Ok(None)
            }
            ParserState::Header { header } => {
                let _ = header.push(byte);
                if header.len() == 6 {
                    let h = Header::read_le(&mut Cursor::new(&header)).map_err(|e| Error::BinRWError(e))?;

                    if h.length % 4 != 0 || h.length < 8 {
                        self.state = ParserState::Sync1;
                        return Ok(None);
                    }

                    if let Messages::Unsupported = h.block_id.message_type() {
                        // Prevents going searching for a
                        // payload/message that we can't deserialize
                        self.state = ParserState::Sync1;
                        return Ok(None);
                    }

                    let payload_len = (h.length - 8) as usize;
                    self.state = ParserState::Payload {
                        header: h,
                        header_buf: header[2..].to_vec(),
                        payload: Vec::with_capacity(payload_len),
                    };
                }
                Ok(None)
            }
            ParserState::Payload { header, header_buf, payload } => {
                payload.push(byte);
                if payload.len() == (header.length - 8) as usize {
                    let mut full_block = Vec::with_capacity(header_buf.len() + payload.len());
                    full_block.extend_from_slice(header_buf);
                    full_block.extend_from_slice(payload);
                    let crc = State::<XMODEM>::calculate(full_block.as_slice());

                    if header.crc != crc {
                        self.state = ParserState::Sync1;
                        return Err(Error::InvalidHeaderCRC);
                    }

                    let res = match header.block_id.message_type() {
                        Messages::QualityInd(_) => {
                            let mut body_cursor = Cursor::new(payload.as_slice());
                            let quality_data = QualityInd::read_le(&mut body_cursor).map_err(|e| Error::BinRWError(e))?;
                            Some(Messages::QualityInd(Some(quality_data)))
                        }
                        Messages::INSNavGeod(_) => {
                            let mut body_cursor = Cursor::new(payload.as_slice());
                            let ins_nav_geod = INSNavGeod::read_le(&mut body_cursor).map_err(|e| Error::BinRWError(e))?;
                            Some(Messages::INSNavGeod(Some(ins_nav_geod)))
                        }
                        Messages::AttEuler(_) => {
                            let mut body_cursor = Cursor::new(payload.as_slice());
                            let att_euler = AttEuler::read_le(&mut body_cursor).map_err(|e| Error::BinRWError(e))?;
                            Some(Messages::AttEuler(Some(att_euler)))
                        }
                        Messages::ExtSensorMeas(_) => {
                            let mut body_cursor = Cursor::new(payload.as_slice());
                            let ext_sensor_meas = ExtSensorMeas::read_le(&mut body_cursor).map_err(|e| Error::BinRWError(e))?;
                            Some(Messages::ExtSensorMeas(Some(ext_sensor_meas)))
                        }
                        Messages::Unsupported => {
                            // should never end up in here since we
                            // return None and reset the parser if its
                            // an unsupported message
                            panic!("Tried to parse an unsupported message!");
                        }
                    };

                    self.state = ParserState::Sync1;
                    return Ok(res);
                }
                Ok(None)
            }
        }
    }
}


#[cfg(test)]

mod tests {
    use super::*;
    use proptest::prelude::*;
    use alloc::vec::Vec;

    const VALID_SYNC: &[u8; 2] = &[36, 64];
    const VALID_QUALITY_IND_HEADER: &[u8; 6] = &[134, 98, 242, 15, 32, 0];
    const VALID_QUALITY_IND_PAYLOAD: &[u8; 24] = &[184, 244, 58, 29, 56, 9, 7, 0, 11, 10, 12, 10, 1, 0, 2, 0, 21, 10, 31, 0, 0, 0, 0, 0];

    const VALID_QUALITY_IND: QualityInd = QualityInd {
        tow: Some(490403000),
        wnc: Some(2360),
        n: 7,
        reserved: 0,
        indicator_1: Some(2571),
        indicator_2: Some(2572),
        indicator_3: Some(1),
        indicator_4: Some(2),
        indicator_5: Some(2581),
        indicator_6: Some(31),
        indicator_7: Some(0),
        indicator_8: None
    };

    proptest! {
        #[test]
        fn test_valid_message_with_noise(noise in proptest::collection::vec(any::<u8>(), 0..10000)) {
            let mut valid_msg = Vec::new();
            valid_msg.extend_from_slice(VALID_SYNC);
            valid_msg.extend_from_slice(VALID_QUALITY_IND_HEADER);
            valid_msg.extend_from_slice(VALID_QUALITY_IND_PAYLOAD);

            let insert_index = if noise.is_empty() { 0 } else { noise.len() / 2 };

            let mut test_input = Vec::new();
            test_input.extend_from_slice(&noise[..insert_index]);
            test_input.extend_from_slice(&valid_msg);
            test_input.extend_from_slice(&noise[insert_index..]);

            // Initialize parser.
            let mut parser = SbfParser::new();
            let mut remaining = test_input.as_slice();
            let mut found = false;

            // Process the input.
            while !remaining.is_empty() {
                match parser.consume(remaining) {
                    Ok((Some(message), _bytes_consumed)) => {
                        if let Messages::QualityInd(Some(qi)) = message {
                            prop_assert_eq!(qi, VALID_QUALITY_IND);
                        } else {
                            prop_assert!(false, "Parsed to wrong Septentrio Message: {:?}", message);
                        }
                        // Additional invariants can be asserted here.
                        found = true;
                        break;
                    },
                    Ok((None, bytes_consumed)) => {
                        remaining = &remaining[bytes_consumed..];
                    },
                    Err(err) => {
                        // If the parser returns an error, that's a failure.
                        prop_assert!(false, "Parser error: {:?}", err);
                    }
                }
            }
            prop_assert!(found, "Valid message was not found in the noise.");
        }
    }
}
