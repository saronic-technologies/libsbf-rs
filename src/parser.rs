extern crate alloc;
use alloc::vec::Vec;

use binrw::io::Cursor;
use binrw::BinRead;

use crate::{Header, MessageKind, Messages, MeasEpoch, MeasExtra, DiffCorrIn, AttEuler, INSNavGeod, ExtSensorMeas, QualityInd, ImuSetup, ReceiverSetup};

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

    // Note: We'll handle unsupported blocks below, no need to reject them here

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
        MessageKind::MeasExtra => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let meas_extra = MeasExtra::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::MeasExtra(meas_extra)
        }
        MessageKind::MeasEpoch => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let meas_epoch = MeasEpoch::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::MeasEpoch(meas_epoch)
        }
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
        MessageKind::DiffCorrIn => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let diff_corr_in = DiffCorrIn::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::DiffCorrIn(diff_corr_in)
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
        MessageKind::ReceiverSetup => {
            let mut body_cursor = Cursor::new(payload.as_slice());
            let receiver_setup = ReceiverSetup::read_le(&mut body_cursor).map_err(|_| ParseError::InvalidPayload)?;
            Messages::ReceiverSetup(receiver_setup)
        }
        MessageKind::Unsupported => {
            debug!("Unsupported block ID: {:#04X}", h.block_id.block_number());
            Messages::Unsupported
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

    // Helper function to create ReceiverSetup test payload
    fn create_receiver_setup_payload() -> Vec<u8> {
        let mut payload = Vec::new();
        
        // TOW and WNc
        payload.extend_from_slice(&490403000u32.to_le_bytes());
        payload.extend_from_slice(&2360u16.to_le_bytes());
        payload.extend_from_slice(&[0u8; 2]); // Reserved
        
        // String fields
        let mut marker_name = [0u8; 60];
        marker_name[..11].copy_from_slice(b"TEST_MARKER");
        payload.extend_from_slice(&marker_name);
        
        let mut marker_number = [0u8; 20];
        marker_number[..5].copy_from_slice(b"12345");
        payload.extend_from_slice(&marker_number);
        
        let mut observer = [0u8; 20];
        observer[..9].copy_from_slice(b"OBSERVER1");
        payload.extend_from_slice(&observer);
        
        let mut agency = [0u8; 40];
        agency[..11].copy_from_slice(b"TEST_AGENCY");
        payload.extend_from_slice(&agency);
        
        let mut rx_serial = [0u8; 20];
        rx_serial[..8].copy_from_slice(b"RX123456");
        payload.extend_from_slice(&rx_serial);
        
        let mut rx_name = [0u8; 20];
        rx_name[..6].copy_from_slice(b"MOSAIC");
        payload.extend_from_slice(&rx_name);
        
        let mut rx_version = [0u8; 20];
        rx_version[..5].copy_from_slice(b"1.0.0");
        payload.extend_from_slice(&rx_version);
        
        let mut ant_serial = [0u8; 20];
        ant_serial[..6].copy_from_slice(b"ANT001");
        payload.extend_from_slice(&ant_serial);
        
        let mut ant_type = [0u8; 20];
        ant_type[..10].copy_from_slice(b"CHOKE_RING");
        payload.extend_from_slice(&ant_type);
        
        // Delta values
        payload.extend_from_slice(&0.0f32.to_le_bytes());
        payload.extend_from_slice(&0.0f32.to_le_bytes());
        payload.extend_from_slice(&0.0f32.to_le_bytes());
        
        let mut marker_type = [0u8; 20];
        marker_type[..8].copy_from_slice(b"GEODETIC");
        payload.extend_from_slice(&marker_type);
        
        let mut fw_version = [0u8; 40];
        fw_version[..7].copy_from_slice(b"FW_V1.0");
        payload.extend_from_slice(&fw_version);
        
        let mut product_name = [0u8; 40];
        product_name[..9].copy_from_slice(b"MOSAIC-X5");
        payload.extend_from_slice(&product_name);
        
        // Position
        payload.extend_from_slice(&0.8997f64.to_le_bytes());
        payload.extend_from_slice(&(-0.00223f64).to_le_bytes());
        payload.extend_from_slice(&45.0f32.to_le_bytes());
        
        let mut station_code = [0u8; 10];
        station_code[..5].copy_from_slice(b"STAT1");
        payload.extend_from_slice(&station_code);
        
        payload.push(1); // MonumentIdx
        payload.push(1); // ReceiverIdx
        payload.extend_from_slice(b"GBR"); // CountryCode
        payload.extend_from_slice(&[0u8; 21]); // Reserved1
        
        payload
    }

    fn create_valid_receiver_setup_message() -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(VALID_SYNC);
        
        let payload = create_receiver_setup_payload();
        let block_id = 5902u16;
        let length = (payload.len() + 8) as u16;
        
        let mut crc_data = Vec::new();
        crc_data.extend_from_slice(&block_id.to_le_bytes());
        crc_data.extend_from_slice(&length.to_le_bytes());
        crc_data.extend_from_slice(&payload);
        let crc = State::<XMODEM>::calculate(&crc_data);
        
        message.extend_from_slice(&crc.to_le_bytes());
        message.extend_from_slice(&block_id.to_le_bytes());
        message.extend_from_slice(&length.to_le_bytes());
        message.extend_from_slice(&payload);
        
        message
    }

    #[test]
    fn test_receiver_setup_parsing() {
        let message = create_valid_receiver_setup_message();
        let mut parser = SbfParser::new();
        
        match parser.consume(&message) {
            Some(Messages::ReceiverSetup(setup)) => {
                assert_eq!(setup.tow, Some(490403000));
                assert_eq!(setup.wnc, Some(2360));
                assert_eq!(&setup.marker_name[..11], b"TEST_MARKER");
                assert_eq!(&setup.marker_number[..5], b"12345");
                assert_eq!(&setup.observer[..9], b"OBSERVER1");
                assert_eq!(&setup.agency[..11], b"TEST_AGENCY");
                assert_eq!(&setup.rx_serial_number[..8], b"RX123456");
                assert_eq!(&setup.rx_name[..6], b"MOSAIC");
                assert_eq!(&setup.rx_version[..5], b"1.0.0");
                assert_eq!(&setup.ant_serial_nbr[..6], b"ANT001");
                assert_eq!(&setup.ant_type[..10], b"CHOKE_RING");
                assert_eq!(setup.delta_h, Some(0.0));
                assert_eq!(setup.delta_e, Some(0.0));
                assert_eq!(setup.delta_n, Some(0.0));
                assert_eq!(&setup.marker_type[..8], b"GEODETIC");
                assert_eq!(&setup.gnss_fw_version[..7], b"FW_V1.0");
                assert_eq!(&setup.product_name[..9], b"MOSAIC-X5");
                assert!(setup.latitude.is_some());
                assert!(setup.longitude.is_some());
                assert_eq!(setup.height, Some(45.0));
                assert_eq!(&setup.station_code[..5], b"STAT1");
                assert_eq!(setup.monument_idx, 1);
                assert_eq!(setup.receiver_idx, 1);
                assert_eq!(&setup.country_code, b"GBR");
            }
            Some(other) => panic!("Expected ReceiverSetup, got {:?}", other),
            None => panic!("Failed to parse ReceiverSetup message"),
        }
    }

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

        #[test]
        fn test_receiver_setup_with_noise(noise in proptest::collection::vec(any::<u8>(), 0..10000)) {
            let valid_msg = create_valid_receiver_setup_message();
            
            let insert_index = if noise.is_empty() { 0 } else { noise.len() / 2 };
            
            let mut test_input = Vec::new();
            test_input.extend_from_slice(&noise[..insert_index]);
            test_input.extend_from_slice(&valid_msg);
            test_input.extend_from_slice(&noise[insert_index..]);
            
            // Initialize parser.
            let mut parser = SbfParser::new();
            
            // Process the input.
            match parser.consume(test_input.as_slice()) {
                Some(Messages::ReceiverSetup(setup)) => {
                    // Just verify key fields to ensure message was parsed correctly
                    prop_assert_eq!(setup.tow, Some(490403000));
                    prop_assert_eq!(setup.wnc, Some(2360));
                    prop_assert_eq!(&setup.marker_name[..11], b"TEST_MARKER");
                    prop_assert_eq!(setup.monument_idx, 1);
                },
                Some(other) => {
                    prop_assert!(false, "Parsed to wrong message type: {:?}", other);
                }
                None => {
                    prop_assert!(false, "Valid ReceiverSetup message was not found in the noise.");
                }
            }
        }
    }
}
