use super::common::ConnectionPort;
use crate::util::binrw::{
    map_f4, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_u1_enum, unmap_u2, unmap_u4,
};
use binrw::binrw;
use serde::Serialize;

// IMU Setup Block 4224
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ImuSetup {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    _reserved: u8,
    /// Connection of the IMU: COM1 to COM4 for an external IMU, or the internal SPI bus
    /// for the internal IMU.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub serial_port: ConnectionPort,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_x_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_y_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_z_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_x_deg: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_y_deg: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_z_deg: Option<f32>,
    /// Alignment parameters are present only in Rev > 0 blocks.
    #[br(try)]
    pub vehicle_alignment: Option<ImuVehicleAlignment>,
}

/// IMU orientation estimated by the receiver and the motion constraint lever
/// arm, both in the vehicle frame.
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ImuVehicleAlignment {
    /// IMU orientation, theta_x angle, estimated by the receiver.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub estimated_theta_x_deg: Option<f32>,
    /// IMU orientation, theta_y angle, estimated by the receiver.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub estimated_theta_y_deg: Option<f32>,
    /// IMU orientation, theta_z angle, estimated by the receiver.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub estimated_theta_z_deg: Option<f32>,
    /// Component of the lever arm from the product reference point to the
    /// motion constraint point along the X-axis of the vehicle frame, as set
    /// with the setVehicleInfo command.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub constraint_lever_arm_x_m: Option<f32>,
    /// Component of the lever arm from the product reference point to the
    /// motion constraint point along the Y-axis of the vehicle frame, as set
    /// with the setVehicleInfo command.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub constraint_lever_arm_y_m: Option<f32>,
    /// Component of the lever arm from the product reference point to the
    /// motion constraint point along the Z-axis of the vehicle frame, as set
    /// with the setVehicleInfo command.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub constraint_lever_arm_z_m: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::ImuSetup;
    use alloc::vec::Vec;
    use binrw::{io::Cursor, BinRead, BinWrite};

    // Body of an IMUSetup block with the given f4 fields after the header fields.
    fn body(values: &[f32]) -> Vec<u8> {
        1000u32
            .to_le_bytes()
            .into_iter()
            .chain(2400u16.to_le_bytes())
            .chain([0, 32])
            .chain(values.iter().flat_map(|x| x.to_le_bytes()))
            .collect()
    }

    // A revision 0 body ends after the orientation angles, so it has no
    // revision 1 fields and writes back unchanged.
    #[test]
    fn reads_revision_0_without_revision_1_fields() {
        let body = body(&[0.1, 0.2, 0.3, 0.0, 90.0, 180.0]);

        let setup = ImuSetup::read_le(&mut Cursor::new(&body)).unwrap();
        assert!(setup.vehicle_alignment.is_none());
        assert_eq!(setup.theta_z_deg, Some(180.0));

        let mut out = Vec::new();
        setup.write_le(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(out, body);
    }

    // A revision 1 body carries the estimated angles and the constraint lever
    // arm, and a Do-Not-Use value among them writes back unchanged.
    #[test]
    fn reads_revision_1_fields() {
        let body = body(&[
            0.1, 0.2, 0.3, 0.0, 90.0, 180.0, 1.0, 89.0, -2e10, 0.5, 0.0, -0.25,
        ]);

        let setup = ImuSetup::read_le(&mut Cursor::new(&body)).unwrap();
        let alignment = setup.vehicle_alignment.as_ref().unwrap();
        assert_eq!(alignment.estimated_theta_y_deg, Some(89.0));
        assert_eq!(alignment.estimated_theta_z_deg, None);
        assert_eq!(alignment.constraint_lever_arm_z_m, Some(-0.25));

        let mut out = Vec::new();
        setup.write_le(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(out, body);
    }
}
