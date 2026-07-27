extern crate alloc;
use alloc::vec::Vec;

use binrw::io::Cursor;
use binrw::BinRead;

use crate::{Header, MessageKind, Messages};

use crc16::*;

use tracing::debug;

#[derive(Debug)]
pub enum Error {
    InvalidHeaderCRC,
    BinRWError(binrw::Error),
}

/// Maximum SBF message size that fits in a single UDP datagram.
/// UDP datagram max = 65535 bytes, minus 8-byte UDP header = 65527 byte payload.
/// SBF messages (including their 8-byte header) must fit within this.
pub const MAX_UDP_PAYLOAD: usize = 65527;

/// Error type for single-datagram parsing via [`parse_datagram`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatagramError {
    /// Not enough data to parse a complete message.
    Incomplete,
    /// No sync sequence ("$@") found in the input.
    NoSync,
    /// CRC validation failed.
    InvalidCrc,
    /// Invalid header (bad length).
    InvalidHeader,
    /// Failed to deserialize the message payload.
    InvalidPayload,
    /// Message length exceeds max UDP payload size (65527 bytes).
    ExceedsMaxUdpPayload(u16),
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

    let msg_kind = h.block_id.message_type();
    if let MessageKind::Unsupported = msg_kind {
        debug!("Unsupported Block ID: {:?}", h.block_id);
        return Ok((
            Messages::Unsupported(h.block_id.block_number()),
            sync_index + total_size,
        ));
    }

    let res = Messages::parse_body(msg_kind, &payload).map_err(|_| ParseError::InvalidPayload)?;

    Ok((res, sync_index + total_size))
}

pub struct SbfParser {
    buf: Vec<u8>,
}

impl Default for SbfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SbfParser {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
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
                }
                Err(ParseError::IncompleteData) => {
                    debug!("Incomplete Data, feed us more!");
                    return None;
                }
                Err(
                    ParseError::InvalidCRC
                    | ParseError::InvalidHeader
                    | ParseError::InvalidPayload
                    | ParseError::SyncNotFound,
                ) => {
                    debug!("Parse error, drain the buffer down");
                    if !self.buf.is_empty() {
                        self.buf.drain(0..1);
                    }
                }
            }
        }
    }
}

/// Parse a single SBF message from a datagram buffer.
///
/// This is designed for UDP datagrams where each packet contains exactly one
/// complete SBF message. Unlike [`SbfParser`] which handles streaming data,
/// this function expects the sync sequence to be at the start of the buffer.
///
/// # Example
///
/// ```no_run
/// use libsbf::parser::parse_datagram;
/// use std::net::UdpSocket;
///
/// let socket = UdpSocket::bind("0.0.0.0:28785").unwrap();
/// let mut buf = [0u8; 65535];
///
/// loop {
///     let (len, _src) = socket.recv_from(&mut buf).unwrap();
///     match parse_datagram(&buf[..len]) {
///         Ok(msg) => println!("{:?}", msg),
///         Err(e) => eprintln!("Parse error: {:?}", e),
///     }
/// }
/// ```
pub fn parse_datagram(datagram: &[u8]) -> core::result::Result<Messages, DatagramError> {
    const MIN_MESSAGE_SIZE: usize = 8; // 2 sync + 2 crc + 2 block_id + 2 length

    if datagram.len() < MIN_MESSAGE_SIZE {
        return Err(DatagramError::Incomplete);
    }

    // Expect sync at start for datagrams
    if &datagram[0..2] != b"$@" {
        return Err(DatagramError::NoSync);
    }

    // Parse header
    let header_slice = &datagram[2..8];
    let h = Header::read_le(&mut Cursor::new(header_slice))
        .map_err(|_| DatagramError::InvalidHeader)?;

    if h.length % 4 != 0 || h.length < 8 {
        return Err(DatagramError::InvalidHeader);
    }

    if h.length as usize > MAX_UDP_PAYLOAD {
        return Err(DatagramError::ExceedsMaxUdpPayload(h.length));
    }

    // Check we have the full message
    let total_len = h.length as usize;
    if datagram.len() < total_len {
        return Err(DatagramError::Incomplete);
    }

    // Validate CRC (covers block_id + length + payload)
    let crc_data = &datagram[4..total_len];
    let calculated_crc = State::<XMODEM>::calculate(crc_data);
    if h.crc != calculated_crc {
        return Err(DatagramError::InvalidCrc);
    }

    let msg_kind = h.block_id.message_type();
    if let MessageKind::Unsupported = msg_kind {
        return Ok(Messages::Unsupported(h.block_id.block_number()));
    }

    // Parse payload
    let payload = &datagram[8..total_len];
    let msg = Messages::parse_body(msg_kind, payload).map_err(|_| DatagramError::InvalidPayload)?;

    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::{parse_datagram, DatagramError, SbfParser};
    use crate::{DOP, Messages, QualityInd, QualityIndicator};
    use alloc::vec::Vec;
    use crc16::{State, XMODEM};
    use proptest::prelude::*;

    const VALID_SYNC: &[u8; 2] = &[36, 64];
    const VALID_QUALITY_IND_HEADER: &[u8; 6] = &[134, 98, 242, 15, 32, 0];
    const VALID_QUALITY_IND_PAYLOAD: &[u8; 24] = &[
        184, 244, 58, 29, 56, 9, 7, 0, 11, 10, 12, 10, 1, 0, 2, 0, 21, 10, 31, 0, 0, 0, 0, 0,
    ];

    /// Frame a complete SBF message: `$@` sync + XMODEM CRC + block_id +
    /// length + payload. The length field is `payload.len() + 8`, which the parser
    /// requires to be a multiple of 4. Sync and CRC are not counted.
    fn build_sbf_message(block_id: u16, payload: &[u8]) -> Vec<u8> {
        let length = (payload.len() + 8) as u16;

        let mut crc_data = Vec::new();
        crc_data.extend_from_slice(&block_id.to_le_bytes());
        crc_data.extend_from_slice(&length.to_le_bytes());
        crc_data.extend_from_slice(payload);
        let crc = State::<XMODEM>::calculate(&crc_data);

        let mut message = Vec::new();
        message.extend_from_slice(VALID_SYNC);
        message.extend_from_slice(&crc.to_le_bytes());
        message.extend_from_slice(&block_id.to_le_bytes());
        message.extend_from_slice(&length.to_le_bytes());
        message.extend_from_slice(payload);
        message
    }

    fn assert_valid_quality_ind(qi: &QualityInd) {
        assert_eq!(qi.tow, Some(490403000));
        assert_eq!(qi.wnc, Some(2360));
        let expected: Vec<QualityIndicator> = [2571u16, 2572, 1, 2, 2581, 31, 0].map(QualityIndicator::from).to_vec();
        assert_eq!(qi.indicators, expected);
    }

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
        build_sbf_message(5902, &create_receiver_setup_payload())
    }

    // Helper function to create a DOP (block 4001) test payload (24 bytes).
    fn create_dop_payload() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&490403000u32.to_le_bytes()); // tow
        payload.extend_from_slice(&2360u16.to_le_bytes()); // wnc
        payload.push(8); // nr_sv
        payload.push(0); // reserved
        payload.extend_from_slice(&150u16.to_le_bytes()); // pdop (1.50)
        payload.extend_from_slice(&120u16.to_le_bytes()); // tdop
        payload.extend_from_slice(&90u16.to_le_bytes()); // hdop
        payload.extend_from_slice(&110u16.to_le_bytes()); // vdop
        payload.extend_from_slice(&12.5f32.to_le_bytes()); // hpl
        payload.extend_from_slice(&20.0f32.to_le_bytes()); // vpl
        payload
    }

    fn assert_valid_dop(dop: &DOP) {
        assert_eq!(dop.tow, Some(490403000));
        assert_eq!(dop.wnc, Some(2360));
        assert_eq!(dop.nr_sv, Some(8));
        assert_eq!(dop.pdop, Some(150));
    }

    /// A known-good message fixture, used to build and identify messages in the
    /// multi-message proptest.
    #[derive(Debug, Clone, Copy)]
    enum TestMsg {
        QualityInd,
        ReceiverSetup,
        Dop,
    }

    impl TestMsg {
        /// A complete, valid framed SBF message for this fixture.
        fn bytes(self) -> Vec<u8> {
            match self {
                TestMsg::QualityInd => build_sbf_message(4082, VALID_QUALITY_IND_PAYLOAD),
                TestMsg::ReceiverSetup => create_valid_receiver_setup_message(),
                TestMsg::Dop => build_sbf_message(4001, &create_dop_payload()),
            }
        }

        /// True if `m` is the message variant this fixture produces, with its key
        /// fields intact. Panics with a precise message if the payload is wrong.
        fn matches(self, m: &Messages) -> bool {
            match (self, m) {
                (TestMsg::QualityInd, Messages::QualityInd(qi)) => {
                    assert_valid_quality_ind(qi);
                    true
                }
                (TestMsg::ReceiverSetup, Messages::ReceiverSetup(rs)) => {
                    assert_eq!(rs.tow, Some(490403000));
                    assert_eq!(&rs.marker_name[..11], b"TEST_MARKER");
                    true
                }
                (TestMsg::Dop, Messages::DOP(dop)) => {
                    assert_valid_dop(dop);
                    true
                }
                _ => false,
            }
        }

        fn strategy() -> impl Strategy<Value = TestMsg> {
            prop_oneof![
                Just(TestMsg::QualityInd),
                Just(TestMsg::ReceiverSetup),
                Just(TestMsg::Dop),
            ]
        }
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

    /// Sanitize noise by making any $@ sequences have invalid headers.
    /// This ensures the parser rejects fake syncs immediately rather than
    /// waiting for more data due to a large claimed message length.
    fn sanitize_noise(mut v: Vec<u8>) -> Vec<u8> {
        for i in 0..v.len().saturating_sub(7) {
            if v[i] == 36 && v[i + 1] == 64 {
                // Set length bytes (offset 6-7 from sync) to invalid value
                // Length of 1 fails the `length >= 8` check
                v[i + 6] = 1;
                v[i + 7] = 0;
            }
        }
        v
    }

    #[test]
    fn test_parse_datagram_valid() {
        let mut datagram = Vec::new();
        datagram.extend_from_slice(VALID_SYNC);
        datagram.extend_from_slice(VALID_QUALITY_IND_HEADER);
        datagram.extend_from_slice(VALID_QUALITY_IND_PAYLOAD);

        let result = parse_datagram(&datagram);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        if let Ok(Messages::QualityInd(qi)) = result {
            assert_valid_quality_ind(&qi);
        } else {
            panic!("Expected QualityInd message, got {:?}", result);
        }
    }

    #[test]
    fn test_parse_datagram_no_sync() {
        let datagram = b"XX\x00\x00\x00\x00\x00\x00";
        let result = parse_datagram(datagram);
        assert!(matches!(result, Err(DatagramError::NoSync)));
    }

    #[test]
    fn test_parse_datagram_incomplete() {
        let datagram = b"$@\x00\x00"; // Only 4 bytes, need at least 8
        let result = parse_datagram(datagram);
        assert!(matches!(result, Err(DatagramError::Incomplete)));
    }

    #[test]
    fn test_parse_datagram_bad_crc() {
        let mut datagram = Vec::new();
        datagram.extend_from_slice(VALID_SYNC);
        datagram.extend_from_slice(VALID_QUALITY_IND_HEADER);
        datagram.extend_from_slice(VALID_QUALITY_IND_PAYLOAD);
        // Corrupt the CRC (bytes 2-3)
        datagram[2] = 0xFF;
        datagram[3] = 0xFF;

        let result = parse_datagram(&datagram);
        assert!(matches!(result, Err(DatagramError::InvalidCrc)));
    }

    #[test]
    fn test_parse_datagram_exceeds_max_udp() {
        // Craft a header with length > MAX_UDP_PAYLOAD (65527)
        // Length field is at bytes 6-7 (little endian)
        let length: u16 = 65528; // Just over the limit
        let block_id: u16 = 4082; // QualityInd

        let mut datagram = Vec::new();
        datagram.extend_from_slice(b"$@"); // sync
        datagram.extend_from_slice(&[0x00, 0x00]); // CRC (doesn't matter, length check comes first)
        datagram.extend_from_slice(&block_id.to_le_bytes());
        datagram.extend_from_slice(&length.to_le_bytes());

        let result = parse_datagram(&datagram);
        assert!(matches!(result, Err(DatagramError::ExceedsMaxUdpPayload(65528))));
    }

    #[test]
    fn test_parse_datagram_unsupported_block_returns_ok() {
        // Block 1000 is in the SBF range but not in MessageKind.
        let block_id: u16 = 1000;
        let payload = [0u8; 8]; // arbitrary, length must be multiple of 4
        let length: u16 = (payload.len() + 8) as u16;

        let mut crc_data = Vec::new();
        crc_data.extend_from_slice(&block_id.to_le_bytes());
        crc_data.extend_from_slice(&length.to_le_bytes());
        crc_data.extend_from_slice(&payload);
        let crc = State::<XMODEM>::calculate(&crc_data);

        let mut datagram = Vec::new();
        datagram.extend_from_slice(b"$@");
        datagram.extend_from_slice(&crc.to_le_bytes());
        datagram.extend_from_slice(&block_id.to_le_bytes());
        datagram.extend_from_slice(&length.to_le_bytes());
        datagram.extend_from_slice(&payload);

        match parse_datagram(&datagram) {
            Ok(Messages::Unsupported(id)) => assert_eq!(id, block_id),
            other => panic!("expected Ok(Unsupported({block_id})), got {other:?}"),
        }
    }

    #[test]
    fn test_parse_datagram_unsupported_block_bad_crc_errors() {
        let block_id: u16 = 1000;
        let payload = [0u8; 8];
        let length: u16 = (payload.len() + 8) as u16;

        let mut datagram = Vec::new();
        datagram.extend_from_slice(b"$@");
        datagram.extend_from_slice(&[0xFF, 0xFF]); // bad CRC
        datagram.extend_from_slice(&block_id.to_le_bytes());
        datagram.extend_from_slice(&length.to_le_bytes());
        datagram.extend_from_slice(&payload);

        let result = parse_datagram(&datagram);
        assert!(matches!(result, Err(DatagramError::InvalidCrc)));
    }

    proptest! {

        #[test]
        fn test_valid_message_with_noise(noise in proptest::collection::vec(any::<u8>(), 0..10000).prop_map(sanitize_noise)) {
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
                    if let Messages::QualityInd(ref qi) = message {
                        assert_valid_quality_ind(qi);
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
        fn test_receiver_setup_with_noise(noise in proptest::collection::vec(any::<u8>(), 0..10000).prop_map(sanitize_noise)) {
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

        /// Feed a random sequence of several valid messages of mixed types,
        /// surrounded by sanitized noise, and assert the parser returns exactly
        /// those messages, in order.
        #[test]
        fn test_multiple_messages_multiple_types(
            kinds in proptest::collection::vec(TestMsg::strategy(), 1..8),
            leading in proptest::collection::vec(any::<u8>(), 0..2000).prop_map(sanitize_noise),
            trailing in proptest::collection::vec(any::<u8>(), 0..2000).prop_map(sanitize_noise),
        ) {
            // Stream: leading noise + contiguous valid messages + trailing noise.
            // The messages are concatenated directly (no noise between them) so
            // the expected message count is deterministic: each valid message is
            // drained by its exact length, and sanitized noise never yields a
            // spurious message.
            let mut stream = Vec::new();
            stream.extend_from_slice(&leading);
            for k in &kinds {
                stream.extend_from_slice(&k.bytes());
            }
            stream.extend_from_slice(&trailing);

            // Feed everything once, then drain every buffered message.
            let mut parser = SbfParser::new();
            let mut parsed = Vec::new();
            if let Some(m) = parser.consume(&stream) {
                parsed.push(m);
            }
            while let Some(m) = parser.consume(&[]) {
                parsed.push(m);
            }

            prop_assert_eq!(
                parsed.len(),
                kinds.len(),
                "expected {} messages, parsed {}",
                kinds.len(),
                parsed.len()
            );

            for (expected, actual) in kinds.iter().zip(parsed.iter()) {
                prop_assert!(
                    expected.matches(actual),
                    "message mismatch: expected {:?}, got {:?}",
                    expected,
                    actual
                );
            }
        }
    }
}
