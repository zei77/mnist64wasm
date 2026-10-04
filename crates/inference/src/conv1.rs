use crate::params::{
    CONV1_WEIGHTS as WEIGHTS, 
    CONV1_DISTANCE_THRESHOLDS as THRESHOLDS
};

const CONV1_KERNEL_H: usize = 6;
const CONV1_KERNEL_W: usize = 6;
const CONV1_OUT_SIDE_H: usize = 10;
const CONV1_OUT_SIDE_W: usize = 10;
pub const CONV1_INPUT_W: usize = 24;
pub const CONV1_INPUT_H: usize = 24;

fn pack(rows: &[u32; CONV1_INPUT_H], x: usize, y: usize) -> u64 {
    let mut value = 0u64;
    for ky in 0..CONV1_KERNEL_H {
        let bits = ((rows[y + ky] >> x) & 0x3f) as u64;
        value |= bits << (ky * CONV1_KERNEL_W);
    }
    value
}

#[inline(always)]
fn distance(lo: u64, hi: u64, w: u64) -> u32 {
    (lo ^ w).count_ones() + 2 * (hi ^ w).count_ones()
}

#[inline(always)]
fn conv1_pos(plane0: &[u32; 24], plane1: &[u32; 24], x: usize, y: usize) -> u32 {
    let input_lo = pack(plane0, x, y);
    let input_hi = pack(plane1, x, y);

    let mut channels = 0u32;
    for channel in 0..WEIGHTS.len() {
        let d = distance(input_lo, input_hi, WEIGHTS[channel]);
        if d <= THRESHOLDS[channel] as u32{
            channels |= 1u32 << channel;
        }            
    }

    channels
}

pub fn conv1(plane0: &[u32; 24], plane1: &[u32; 24]) -> [u64; 50] {
    let mut output = [0u64; 50];
    let mut out = 0;

    for oy in 0..CONV1_OUT_SIDE_H {
        let y = oy * 2;

        for ox in (0..CONV1_OUT_SIDE_W).step_by(2) {
            let (x0, x1) = (ox * 2, ox * 2 + 2);

            let a = conv1_pos(plane0, plane1, x0, y);
            let b = conv1_pos(plane0, plane1, x1, y);

            output[out] = a as u64 | ((b as u64) << 32);
            out += 1;
        }
    }
    output
}