use crate::conv1::{CONV1_INPUT_H, CONV1_INPUT_W};

pub fn to_bitplanes(input: &[u8; CONV1_INPUT_H * CONV1_INPUT_W]) -> ([u32; CONV1_INPUT_H], [u32; CONV1_INPUT_H]) {
    let mut lo = [0u32; CONV1_INPUT_H];
    let mut hi = [0u32; CONV1_INPUT_H];

    for y in 0..CONV1_INPUT_H {
        let mut low_row = 0u32;
        let mut high_row = 0u32;

        for x in 0..CONV1_INPUT_W {
            let pixel = input[y * CONV1_INPUT_W + x];
            low_row |= ((pixel & 1) as u32) << x;
            high_row |= (((pixel >> 1) & 1) as u32) << x;
        }

        lo[y] = low_row;
        hi[y] = high_row;
    }

    (lo, hi)
}
