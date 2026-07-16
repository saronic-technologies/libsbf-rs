#[cfg(test)]
mod tests {
    use crate::{
        AuxAntPositions, BaseVectorCart, BaseVectorGeod, ChannelStatus, Comment, DiskStatus,
        ExtSensorMeas, MeasEpoch, MeasExtra, Messages, reader::SbfReader, ReceiverStatus, RFStatus,
        RxMessage, SatVisibility,
    };
    use binrw::{io::Cursor, BinRead, BinWrite};
    use std::collections::HashMap;
    use std::fs::{self, File};

    #[test]
    fn test_mega_file_all_message_types() {
        // This test verifies that mega_test.sbf contains all expected message types
        // and that they all parse without errors

        let input_stream = File::open("test-files/mega_test.sbf")
            .expect("Failed to open mega_test.sbf - make sure test file exists");

        let sbf_reader = SbfReader::new(input_stream);

        let mut message_counts: HashMap<String, usize> = HashMap::new();
        let mut total_messages = 0;
        let mut parse_errors = 0;

        for msg_result in sbf_reader {
            match msg_result {
                Ok(msg) => {
                    total_messages += 1;
                    let debug = format!("{msg:?}");
                    let msg_type = debug.split('(').next().unwrap().to_string();
                    *message_counts.entry(msg_type).or_insert(0) += 1;
                }
                Err(e) => {
                    parse_errors += 1;
                    eprintln!("Parse error: {:?}", e);
                }
            }
        }

        // Verify we parsed messages successfully
        assert!(
            total_messages > 0,
            "No messages were parsed from mega_test.sbf"
        );
        assert_eq!(parse_errors, 0, "There were {} parse errors", parse_errors);

        // Expected message types (all implemented types except Unsupported)
        let expected_types = vec![
            "AttCovEuler",
            "AttEuler",
            "BaseVectorGeod",
            "BDSIon",
            "ChannelStatus",
            "Commands",
            "Comment",
            "DiffCorrIn",
            "DiskStatus",
            "EndOfAtt",
            "EndOfMeas",
            "EndOfPVT",
            "ExtSensorInfo",
            "ExtSensorMeas",
            "ExtSensorStatus",
            "GALGstGps",
            "GALIon",
            "GALNav",
            "GALUtc",
            "GEONav",
            "GEORawL1",
            "GPSCNav",
            "GPSIon",
            "GPSNav",
            "GPSUtc",
            "ImuSetup",
            "INSNavGeod",
            "INSSupport",
            "Meas3Doppler",
            "Meas3Ranges",
            "MeasEpoch",
            "MeasExtra",
            "PosCovGeodetic",
            "PVTGeodetic",
            "QualityInd",
            "ReceiverSetup",
            "ReceiverStatus",
            "ReceiverTime",
            "RxMessage",
            "SatVisibility",
            "VelSensorSetup",
        ];

        // Check that we have at least 27 different message types
        // (INSNavGeod and EndOfMeas might be missing if the file wasn't updated)
        let unique_types = message_counts.len();
        assert!(
            unique_types >= 27,
            "Expected at least 27 message types, found only {}. Missing types: {:?}",
            unique_types,
            expected_types
                .iter()
                .filter(|&&t| !message_counts.contains_key(t))
                .collect::<Vec<_>>()
        );

        // Verify specific expected counts for key message types
        assert!(
            message_counts.get("PVTGeodetic").unwrap_or(&0) >= &2,
            "Expected at least 2 PVTGeodetic messages"
        );
        assert!(
            message_counts.get("GALNav").unwrap_or(&0) >= &10,
            "Expected at least 10 GALNav messages"
        );
        assert!(
            message_counts.get("Commands").unwrap_or(&0) >= &5,
            "Expected at least 5 Commands messages"
        );

        // Counts for the other supported blocks in mega_test.sbf.
        assert_eq!(message_counts.get("EndOfAtt").copied().unwrap_or(0), 1, "expected 1 EndOfAtt");
        assert_eq!(message_counts.get("EndOfPVT").copied().unwrap_or(0), 2, "expected 2 EndOfPVT");
        assert_eq!(message_counts.get("ReceiverTime").copied().unwrap_or(0), 2, "expected 2 ReceiverTime");
        assert_eq!(message_counts.get("SatVisibility").copied().unwrap_or(0), 1, "expected 1 SatVisibility");
        assert_eq!(message_counts.get("ChannelStatus").copied().unwrap_or(0), 1, "expected 1 ChannelStatus");
        assert_eq!(message_counts.get("BaseVectorGeod").copied().unwrap_or(0), 1, "expected 1 BaseVectorGeod");
        assert_eq!(message_counts.get("DiskStatus").copied().unwrap_or(0), 1, "expected 1 DiskStatus");
        assert_eq!(message_counts.get("RxMessage").copied().unwrap_or(0), 1, "expected 1 RxMessage");
        assert_eq!(message_counts.get("Comment").copied().unwrap_or(0), 1, "expected 1 Comment");

        println!("Mega test file validation passed!");
        println!("Total messages: {}", total_messages);
        println!("Unique message types: {}", unique_types);

        // Print summary
        let mut sorted_types: Vec<_> = message_counts.iter().collect();
        sorted_types.sort_by(|a, b| a.0.cmp(b.0));

        for (msg_type, count) in sorted_types {
            println!("  {}: {}", msg_type, count);
        }
    }

    #[test]
    fn test_mega_file_message_integrity() {
        // This test spot-checks specific messages to ensure they have valid data

        let input_stream =
            File::open("test-files/mega_test.sbf").expect("Failed to open mega_test.sbf");

        let sbf_reader = SbfReader::new(input_stream);

        let mut found_receiver_setup = false;
        let mut found_gps_nav = false;
        let mut found_ext_sensor = false;
        let mut found_channel_status = false;
        let mut found_sat_visibility = false;
        let mut found_disk_status = false;
        let mut found_receiver_time = false;
        let mut found_comment = false;
        let mut found_meas_epoch = false;

        for msg_result in sbf_reader {
            if let Ok(msg) = msg_result {
                match msg {
                    Messages::ReceiverSetup(setup) => {
                        // Verify ReceiverSetup has expected data
                        assert!(setup.tow.is_some(), "ReceiverSetup should have TOW");
                        assert!(setup.wnc.is_some(), "ReceiverSetup should have WNc");

                        // Check marker name starts with expected prefix
                        let marker_str = std::str::from_utf8(&setup.marker_name[..4]).unwrap_or("");
                        assert_eq!(marker_str, "SEPT", "Expected SEPT marker name prefix");

                        // Verify location is reasonable (Belgium coordinates)
                        if let Some(lat) = setup.latitude {
                            assert!(
                                lat > 0.87 && lat < 0.89,
                                "Latitude should be ~50.8 degrees N"
                            );
                        }

                        found_receiver_setup = true;
                    }
                    Messages::GPSNav(nav) => {
                        // Verify GPS navigation message
                        assert!(nav.tow.is_some(), "GPSNav should have TOW");
                        assert!(nav.prn >= 1 && nav.prn <= 32, "GPS PRN should be 1-32");
                        assert!(
                            nav.sqrt_a > 5000.0 && nav.sqrt_a < 6000.0,
                            "GPS sqrt_a should be ~5153 (GPS orbit)"
                        );
                        found_gps_nav = true;
                    }
                    Messages::ExtSensorMeas(ext) => {
                        // Verify external sensor data
                        assert!(ext.n > 0, "ExtSensorMeas should have sensor data");
                        assert!(ext.sb_length > 0, "ExtSensorMeas should have data length");
                        found_ext_sensor = true;
                    }
                    Messages::ChannelStatus(cs) => {
                        assert!(cs.tow.is_some(), "ChannelStatus should have TOW");
                        assert_eq!(
                            cs.sat_info.len(),
                            usize::from(cs.n),
                            "ChannelStatus N should match its sub-block count"
                        );
                        if let Some(sat) = cs.sat_info.first() {
                            assert_eq!(
                                sat.state_info.len(),
                                usize::from(sat.n2),
                                "ChannelSatInfo N2 should match its state sub-block count"
                            );
                        }
                        found_channel_status = true;
                    }
                    Messages::SatVisibility(sv) => {
                        assert!(sv.tow.is_some(), "SatVisibility should have TOW");
                        assert_eq!(
                            sv.satellites.len(),
                            usize::from(sv.n),
                            "SatVisibility N should match its sub-block count"
                        );
                        found_sat_visibility = true;
                    }
                    Messages::DiskStatus(ds) => {
                        assert!(ds.tow.is_some(), "DiskStatus should have TOW");
                        assert_eq!(
                            ds.disks.len(),
                            usize::from(ds.n),
                            "DiskStatus N should match its sub-block count"
                        );
                        found_disk_status = true;
                    }
                    Messages::ReceiverTime(rt) => {
                        assert!(rt.tow.is_some(), "ReceiverTime should have TOW");
                        assert!(rt.utc_year.is_some(), "ReceiverTime should have a UTC year");
                        found_receiver_time = true;
                    }
                    Messages::Comment(c) => {
                        assert_eq!(
                            c.comment.len(),
                            usize::from(c.comment_ln),
                            "Comment length should match its byte count"
                        );
                        found_comment = true;
                    }
                    Messages::MeasEpoch(me) => {
                        assert!(me.tow.is_some(), "MeasEpoch should have TOW");
                        assert_eq!(
                            me.channel_type1.len(),
                            usize::from(me.n1),
                            "MeasEpoch N1 should match its sub-block count"
                        );
                        if let Some(ct1) = me.channel_type1.first() {
                            assert_eq!(
                                ct1.channel_type2.len(),
                                usize::from(ct1.n2),
                                "ChannelType1 N2 should match its sub-block count"
                            );
                        }
                        found_meas_epoch = true;
                    }
                    _ => {}
                }
            }
        }

        assert!(found_receiver_setup, "Should find ReceiverSetup message");
        assert!(found_gps_nav, "Should find GPSNav message");
        assert!(found_ext_sensor, "Should find ExtSensorMeas message");
        assert!(found_channel_status, "Should find ChannelStatus message");
        assert!(found_sat_visibility, "Should find SatVisibility message");
        assert!(found_disk_status, "Should find DiskStatus message");
        assert!(found_receiver_time, "Should find ReceiverTime message");
        assert!(found_comment, "Should find Comment message");
        assert!(found_meas_epoch, "Should find MeasEpoch message");
    }

    /// Reads every binrw block from the mega file, writes it back, and checks the
    /// output reproduces the body up to any trailing padding. Padding is undefined
    /// per SBF 4.1.5, so it is not reproduced.
    #[test]
    fn test_mega_file_roundtrip() {
        fn round_trip<T>(body: &[u8], block: u16, count: &mut usize)
        where
            for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
        {
            let value = T::read_le(&mut Cursor::new(body))
                .unwrap_or_else(|e| panic!("block {block} failed to read: {e:?}"));
            let mut out = Vec::new();
            value
                .write_le(&mut Cursor::new(&mut out))
                .unwrap_or_else(|e| panic!("block {block} failed to write: {e:?}"));
            assert!(
                out.len() <= body.len(),
                "block {block} wrote {} bytes, more than its {} byte body",
                out.len(),
                body.len()
            );
            assert_eq!(&out[..], &body[..out.len()], "block {block} did not round-trip");
            *count += 1;
        }

        let data = fs::read("test-files/mega_test.sbf").expect("read mega_test.sbf");
        let mut present: HashMap<u16, usize> = HashMap::new();
        let mut round_tripped = 0usize;

        let mut i = 0;
        while i + 8 <= data.len() {
            if data[i] != 0x24 || data[i + 1] != 0x40 {
                i += 1;
                continue;
            }
            let ident = u16::from_le_bytes([data[i + 4], data[i + 5]]);
            let length = usize::from(u16::from_le_bytes([data[i + 6], data[i + 7]]));
            if length < 8 || length % 4 != 0 || i + length > data.len() {
                i += 1;
                continue;
            }
            let block = ident & 0x1FFF;
            let body = &data[i + 8..i + length];
            match block {
                4000 => round_trip::<MeasExtra>(body, block, &mut round_tripped),
                4012 => round_trip::<SatVisibility>(body, block, &mut round_tripped),
                4013 => round_trip::<ChannelStatus>(body, block, &mut round_tripped),
                4014 => round_trip::<ReceiverStatus>(body, block, &mut round_tripped),
                4027 => round_trip::<MeasEpoch>(body, block, &mut round_tripped),
                4028 => round_trip::<BaseVectorGeod>(body, block, &mut round_tripped),
                4043 => round_trip::<BaseVectorCart>(body, block, &mut round_tripped),
                4050 => round_trip::<ExtSensorMeas>(body, block, &mut round_tripped),
                4059 => round_trip::<DiskStatus>(body, block, &mut round_tripped),
                4092 => round_trip::<RFStatus>(body, block, &mut round_tripped),
                4103 => round_trip::<RxMessage>(body, block, &mut round_tripped),
                5936 => round_trip::<Comment>(body, block, &mut round_tripped),
                5942 => round_trip::<AuxAntPositions>(body, block, &mut round_tripped),
                _ => {}
            }
            *present.entry(block).or_insert(0) += 1;
            i += length;
        }

        assert!(round_tripped > 0, "no binrw blocks were round-tripped");
        assert!(present.contains_key(&4013), "ChannelStatus should be present");
        assert!(present.contains_key(&4027), "MeasEpoch should be present");
    }
}
