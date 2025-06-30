extern crate alloc;
use alloc::vec::Vec;

use binrw::io::Cursor;
use binrw::BinRead;

use crate::{Header, MessageKind, Messages, AttEuler, INSNavGeod, ExtSensorMeas, QualityInd, ImuSetup};

use crc16::*;

use tracing::debug;

#[derive(Debug)]
pub enum Error {
    InvalidHeaderCRC,
    BinRWError(binrw::Error),
}


enum ParseError {
    IncompleteData,
    InvalidHeader,
    InvalidCRC,
    InvalidPayload,
    SyncNotFound,
}

type Result<T> = core::result::Result<(T, usize), ParseError>;

// Constants for our parser.
const MIN_MESSAGE_SIZE: usize = 8; // 2 sync bytes + 6 header bytes

fn parse_message(input: &[u8]) -> Result<Messages> {
    // Make sure the input isn't empty
    if input.is_empty() {
        debug!("Incomplete data, don't have enough for sync");
        return Err(ParseError::IncompleteData);
    }

    // Find the sync sequence "$@".
    let sync_index = input
        .windows(2)
        .position(|w| w == b"$@")
        .ok_or(ParseError::SyncNotFound)?;

    // Make sure there's enough data for sync, header, and payload.
    if input.len() < sync_index + MIN_MESSAGE_SIZE {
        debug!("Incomplete data, don't have enough for sync and header");
        return Err(ParseError::IncompleteData);
    }

    // Extract and validate the header.
    let header_start = sync_index + 2;
    let header_end = header_start + 6;
    let header_slice = &input[header_start..header_end];
    let header: [u8; 6] = header_slice.try_into().unwrap();

    let h = Header::read_le(&mut Cursor::new(&header)).map_err(|_| ParseError::InvalidHeader)?;
    if h.length % 4 != 0 || h.length < 8 {
        debug!("Invalid header length: {}", h.length);
        return Err(ParseError::InvalidHeader);
    }

    if let MessageKind::Unsupported = h.block_id.message_type() {
        debug!("Unsupported Block ID: {:?}", h.block_id);
        return Err(ParseError::InvalidHeader);
    }

    // Ensure we have the complete payload.
    let total_size = 2 + 6 + (h.length as usize) - 8;
    if input.len() < sync_index + total_size {
        debug!("Don't have full message.");
        return Err(ParseError::IncompleteData);
    }

    // Build the message.
    let payload_start = header_end;
    let payload = input[payload_start..payload_start + (h.length as usize) - 8].to_vec();
    let mut full_block = Vec::with_capacity(4 + payload.len());
    full_block.extend_from_slice(&header[2..]);
    full_block.extend_from_slice(&payload);
    let crc = State::<XMODEM>::calculate(full_block.as_slice());

    if h.crc != crc {
        debug!("Invalid CRC for {:?}", h.block_id.message_type());
        return Err(ParseError::InvalidCRC);
    }

    let res = match h.block_id.message_type() {
        MessageKind::QualityInd => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let quality_data = QualityInd::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::QualityInd(quality_data)
        }
        MessageKind::INSNavGeod => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let ins_nav_geod = INSNavGeod::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::INSNavGeod(ins_nav_geod)
        }
        MessageKind::AttEuler => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let att_euler = AttEuler::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::AttEuler(att_euler)
        }
        MessageKind::ExtSensorMeas => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let ext_sensor_meas = ExtSensorMeas::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::ExtSensorMeas(ext_sensor_meas)
        }
        MessageKind::ImuSetup => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let imu_setup = ImuSetup::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::ImuSetup(imu_setup)
        }
        _ => {
            // should never end up in here since we
            // return None and reset the parser if its
            // an unsupported message
            panic!("Tried to parse an unsupported message!");
        }
    };

    Ok((res, sync_index + total_size))
}

pub struct SbfParser {
    buf: Vec<u8>,
}

impl SbfParser {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }

    /// Consume bytes and attempt to parse the message. If we can't
    /// find a message we return None. If we get a message it doesn't
    /// gurantee the whole buffer internal buffer is drained.
    pub fn consume(&mut self, input: &[u8]) -> Option<Messages> {
        self.buf.extend(input);
        loop {
            debug!("Internal Buffer Size: {}", self.buf.len());
            match parse_message(&self.buf) {
                Ok((msg, bytes_consumed)) => {
                    debug!("draining the buffer");
                    self.buf.drain(0..bytes_consumed);
                    return Some(msg);
                },
                Err(ParseError::IncompleteData) => {
                    debug!("Incomplete Data, feed us more!");
                    return None;
                }
                Err(ParseError::InvalidCRC | ParseError::InvalidHeader | ParseError::InvalidPayload | ParseError::SyncNotFound) => {
                    debug!("Parse error, drain the buffer down");
                    if !self.buf.is_empty() {
                        self.buf.drain(0..1);
                    }
                }
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

            
            // Process the input.
            match parser.consume(test_input.as_slice()) {
                Some(message) => {
                    if let Messages::QualityInd(qi) = message {
                        prop_assert_eq!(qi, VALID_QUALITY_IND);
                    } else {
                        prop_assert!(false, "Parsed to wrong Septentrio Message: {:?}", message);
                    }

                    // Additional invariants can be asserted here.
                    prop_assert!(true, "Valid message was not found in the noise.");
                },
                None => {
                    prop_assert!(false, "Valid message was not found in the noise.");
                }
            }
        }
    }
}
