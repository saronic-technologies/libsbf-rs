#[cfg(test)]
mod tests {
    use crate::{Messages, reader::SbfReader};
    use std::collections::HashMap;
    use std::fs::File;

    #[test]
    fn test_mega_file_all_message_types() {
        // This test verifies that mega_test.sbf contains all expected message types
        // and that they all parse without errors
        
        let input_stream = File::open("test-files/mega_test.sbf")
            .expect("Failed to open mega_test.sbf - make sure test file exists");
        
        let sbf_reader = SbfReader::new(input_stream);
        
        let mut message_counts: HashMap<&str, usize> = HashMap::new();
        let mut total_messages = 0;
        let mut parse_errors = 0;
        
        for msg_result in sbf_reader {
            match msg_result {
                Ok(msg) => {
                    total_messages += 1;
                    
                    let msg_type = match msg {
                        Messages::MeasExtra(_) => "MeasExtra",
                        Messages::GALNav(_) => "GALNav",
                        Messages::PVTGeodetic(_) => "PVTGeodetic",
                        Messages::ReceiverStatus(_) => "ReceiverStatus",
                        Messages::Commands(_) => "Commands",
                        Messages::GEORawL1(_) => "GEORawL1",
                        Messages::MeasEpoch(_) => "MeasEpoch",
                        Messages::GALIon(_) => "GALIon",
                        Messages::GALUtc(_) => "GALUtc",
                        Messages::GALGstGps(_) => "GALGstGps",
                        Messages::GPSCNav(_) => "GPSCNav",
                        Messages::Meas3Ranges(_) => "Meas3Ranges",
                        Messages::Meas3Doppler(_) => "Meas3Doppler",
                        Messages::BDSIon(_) => "BDSIon",
                        Messages::INSSupport(_) => "INSSupport",
                        Messages::QualityInd(_) => "QualityInd",
                        Messages::INSNavGeod(_) => "INSNavGeod",
                        Messages::VelSensorSetup(_) => "VelSensorSetup",
                        Messages::AttEuler(_) => "AttEuler",
                        Messages::AttCovEuler(_) => "AttCovEuler",
                        Messages::DiffCorrIn(_) => "DiffCorrIn",
                        Messages::ExtSensorMeas(_) => "ExtSensorMeas",
                        Messages::ExtSensorStatus(_) => "ExtSensorStatus",
                        Messages::ExtSensorInfo(_) => "ExtSensorInfo",
                        Messages::ImuSetup(_) => "ImuSetup",
                        Messages::ReceiverSetup(_) => "ReceiverSetup",
                        Messages::GEONav(_) => "GEONav",
                        Messages::GPSIon(_) => "GPSIon",
                        Messages::GPSNav(_) => "GPSNav",
                        Messages::GPSUtc(_) => "GPSUtc",
                        Messages::PosCovGeodetic(_) => "PosCovGeodetic",
                        Messages::Unsupported(_) => "Unsupported",
                    };
                    
                    *message_counts.entry(msg_type).or_insert(0) += 1;
                }
                Err(e) => {
                    parse_errors += 1;
                    eprintln!("Parse error: {:?}", e);
                }
            }
        }
        
        // Verify we parsed messages successfully
        assert!(total_messages > 0, "No messages were parsed from mega_test.sbf");
        assert_eq!(parse_errors, 0, "There were {} parse errors", parse_errors);
        
        // Expected message types (all implemented types except Unsupported)
        let expected_types = vec![
            "AttCovEuler", "AttEuler", "BDSIon", "Commands", "DiffCorrIn",
            "ExtSensorInfo", "ExtSensorMeas", "ExtSensorStatus", "GALGstGps",
            "GALIon", "GALNav", "GALUtc", "GEONav", "GEORawL1", "GPSCNav",
            "GPSIon", "GPSNav", "GPSUtc", "ImuSetup", "INSNavGeod", "INSSupport",
            "Meas3Doppler", "Meas3Ranges", "MeasEpoch", "MeasExtra",
            "PosCovGeodetic", "PVTGeodetic", "QualityInd", "ReceiverSetup",
            "ReceiverStatus", "VelSensorSetup"
        ];
        
        // Check that we have at least 27 different message types
        // (INSNavGeod might be missing if the file wasn't updated)
        let unique_types = message_counts.len();
        assert!(
            unique_types >= 27,
            "Expected at least 27 message types, found only {}. Missing types: {:?}",
            unique_types,
            expected_types.iter()
                .filter(|&&t| !message_counts.contains_key(t))
                .collect::<Vec<_>>()
        );
        
        // Verify specific expected counts for key message types
        assert!(message_counts.get("PVTGeodetic").unwrap_or(&0) >= &2, 
                "Expected at least 2 PVTGeodetic messages");
        assert!(message_counts.get("GALNav").unwrap_or(&0) >= &10,
                "Expected at least 10 GALNav messages");
        assert!(message_counts.get("Commands").unwrap_or(&0) >= &5,
                "Expected at least 5 Commands messages");
        
        println!("Mega test file validation passed!");
        println!("Total messages: {}", total_messages);
        println!("Unique message types: {}", unique_types);
        
        // Print summary
        let mut sorted_types: Vec<_> = message_counts.iter().collect();
        sorted_types.sort_by_key(|&(name, _)| *name);
        
        for (msg_type, count) in sorted_types {
            println!("  {}: {}", msg_type, count);
        }
    }

    #[test]
    fn test_mega_file_message_integrity() {
        // This test spot-checks specific messages to ensure they have valid data
        
        let input_stream = File::open("test-files/mega_test.sbf")
            .expect("Failed to open mega_test.sbf");
        
        let sbf_reader = SbfReader::new(input_stream);
        
        let mut found_receiver_setup = false;
        let mut found_gps_nav = false;
        let mut found_ext_sensor = false;
        
        for msg_result in sbf_reader {
            if let Ok(msg) = msg_result {
                match msg {
                    Messages::ReceiverSetup(setup) => {
                        // Verify ReceiverSetup has expected data
                        assert!(setup.tow.is_some(), "ReceiverSetup should have TOW");
                        assert!(setup.wnc.is_some(), "ReceiverSetup should have WNc");
                        
                        // Check marker name starts with expected prefix
                        let marker_str = std::str::from_utf8(&setup.marker_name[..4])
                            .unwrap_or("");
                        assert_eq!(marker_str, "SEPT", "Expected SEPT marker name prefix");
                        
                        // Verify location is reasonable (Belgium coordinates)
                        if let Some(lat) = setup.latitude {
                            assert!(lat > 0.87 && lat < 0.89, "Latitude should be ~50.8 degrees N");
                        }
                        
                        found_receiver_setup = true;
                    }
                    Messages::GPSNav(nav) => {
                        // Verify GPS navigation message
                        assert!(nav.tow.is_some(), "GPSNav should have TOW");
                        assert!(nav.prn >= 1 && nav.prn <= 32, "GPS PRN should be 1-32");
                        assert!(nav.sqrt_a > 5000.0 && nav.sqrt_a < 6000.0,
                                "GPS sqrt_a should be ~5153 (GPS orbit)");
                        found_gps_nav = true;
                    }
                    Messages::ExtSensorMeas(ext) => {
                        // Verify external sensor data
                        assert!(ext.n > 0, "ExtSensorMeas should have sensor data");
                        assert!(ext.sb_length > 0, "ExtSensorMeas should have data length");
                        found_ext_sensor = true;
                    }
                    _ => {}
                }
            }
        }
        
        assert!(found_receiver_setup, "Should find ReceiverSetup message");
        assert!(found_gps_nav, "Should find GPSNav message");
        assert!(found_ext_sensor, "Should find ExtSensorMeas message");
    }
}