use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
const BYTES_PER_PLANE: usize = 24 * 24 / 8;
const BYTES_PER_IMAGE: usize = BYTES_PER_PLANE * 2; // 144
const HEIGHT: usize = 24;

#[inline(always)]
fn to_u32(data: &[u8], i: usize) -> u32 {
    data[i] as u32
    | ((data[i + 1] as u32) << 8)
    | ((data[i + 2] as u32) << 16)
}

pub fn read_image(path: &str, image_index: usize) -> std::io::Result<([u32; 24], [u32; 24])> {
    let mut file = File::open(path)?;

    let offset = (image_index * BYTES_PER_IMAGE) as u64;
    file.seek(SeekFrom::Start(offset))?;

    let mut data = [0u8; BYTES_PER_IMAGE];
    file.read_exact(&mut data)?;

    let mut plane0 = [0u32; HEIGHT];
    let mut plane1 = [0u32; HEIGHT];

    for y in 0..HEIGHT {
        let i = y * 3;
        let j = BYTES_PER_PLANE + i;

        plane0[y] = to_u32(&data, i);
        plane1[y] = to_u32(&data, j);
    }

    Ok((plane0, plane1))
}