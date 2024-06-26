mod utils;

use bitvec::prelude::*;
use core::panic;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

const TRNS: [u8; 4] = [0, 85, 170, 255];

#[wasm_bindgen]
pub fn encode_2bit_indexed_trns_png(
    buf: Vec<u8>,
    width: u32,
    height: u32,
    target_rgb: Vec<u8>,
) -> Vec<u8> {
    set_panic_hook();

    if target_rgb.len() != 3 {
        panic!("Invalid target_rgb length");
    }

    let mut palette: Vec<u8> = Vec::new();
    for _ in 0..4 {
        palette.extend_from_slice(&target_rgb);
    }

    let indices: BitVec<u8, Msb0> = buf
        .chunks_exact(4)
        .flat_map(|chunk| match chunk[3] {
            a if a > 213 => bitvec![1, 1], // TRNS[3]
            a if a > 128 => bitvec![1, 0], // TRNS[2]
            a if a > 42 => bitvec![0, 1],  // TRNS[1]
            _ => bitvec![0, 0],            // TRNS[0]
        })
        .collect();
    let indices = indices.into_vec();

    let mut encoded_data = Vec::<u8>::with_capacity(indices.capacity());

    {
        let mut encoder = png::Encoder::new(&mut encoded_data, width, height);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Two);
        encoder.set_compression(png::Compression::Best);
        encoder.set_filter(png::FilterType::NoFilter);
        encoder.set_palette(palette);
        encoder.set_trns(Vec::from(TRNS));

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&indices).unwrap();
    }

    encoded_data
}
