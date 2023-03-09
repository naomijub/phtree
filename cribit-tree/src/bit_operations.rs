pub const fn get_bit_u(l: u32, pos_bit: u32) -> bool {
    let l = l as u64;
    (l & (0x8000_0000_0000_0000 >> pos_bit)) != 0
}

pub fn get_bit_u_array(ba: &[u32], pos_bit: u32) -> bool {
    let ba: &[u64] = &ba.iter().map(|i| u64::from(*i)).collect::<Vec<u64>>();
    let mut pos_bit = pos_bit;
    let pa = pos_bit >> 6;
    pos_bit &= 0x3F;
    (ba[pa as usize] & (0x8000_0000_0000_0000 >> pos_bit)) != 0
}

pub const fn extract_prefix(v: u32, end_position: u32) -> u32 {
    let mut inf = v;
    if end_position < 63 {
        inf &= !((u32::MAX) >> (1 + end_position)); // & 0x3f == %64
    }
    inf
}

pub const fn does_prefix_match(pos_diff: u32, v: u32, prefix: u32) -> bool {
    if pos_diff > 0 {
        (v ^ prefix) >> (64 - pos_diff) == 0
    } else {
        true
    }
}
