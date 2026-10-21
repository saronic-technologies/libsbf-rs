use crate::util::binrw::{
    map_u2, map_u4, subblock_count_mod256, unmap_u2, unmap_u4, unwrap_subblocks, wrap_subblocks,
};
use alloc::vec::Vec;
use binrw::binrw;

// MeasExtra Block 4000
#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtra {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub doppler_var_factor: f32,
    #[br(temp, parse_with = subblock_count_mod256, args(n, sb_length))]
    #[bw(ignore)]
    count: usize,
    #[br(args { count, inner: (usize::from(sb_length),) }, map = unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = wrap_subblocks)]
    pub channel_sub: Vec<MeasExtraChannelSub>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtraChannelSub {
    pub rx_channel: u8,
    pub type_field: u8,
    pub mp_correction: i16,
    pub smoothing_corr: i16,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub code_var: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub carrier_var: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub lock_time: Option<u16>,
    pub cum_loss_cont: u8,
    pub car_mp_corr: i8,
    pub info: u8,
    pub misc: u8,
}

#[cfg(test)]
mod tests {
    use super::MeasExtra;
    use alloc::vec::Vec;
    use binrw::{io::Cursor, BinRead, BinWrite};

    // N holds 300 modulo 256, so all 300 sub-blocks can be found only through the
    // body length. Each sub-block carries its index as the multipath correction.
    #[test]
    fn reads_more_than_255_sub_blocks() {
        let body: Vec<u8> = 1000u32
            .to_le_bytes()
            .into_iter()
            .chain(2400u16.to_le_bytes())
            .chain([44, 16])
            .chain(1.0f32.to_le_bytes())
            .chain((0..300u16).flat_map(|i| {
                [(i % 256) as u8, 0]
                    .into_iter()
                    .chain((i as i16).to_le_bytes())
                    .chain([0; 12])
            }))
            .collect();

        let block = MeasExtra::read_le(&mut Cursor::new(&body)).unwrap();
        assert_eq!(block.n, 44);
        let corrections: Vec<i16> = block
            .channel_sub
            .iter()
            .map(|sub| sub.mp_correction)
            .collect();
        assert_eq!(corrections, (0..300).collect::<Vec<i16>>());

        let mut out = Vec::new();
        block.write_le(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(out, body);
    }
}
