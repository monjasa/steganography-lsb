use bitvec::order::Lsb0;
use bitvec::view::BitViewSized;
use image::{DynamicImage, ImageBuffer, Rgb};

const LSB_MASK_FALSE: u8 = 0b1111_1110;
const LSB_MASK_TRUE: u8 = 0b0000_0001;

pub fn embed(img: &DynamicImage, msg: &Vec<u8>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let msg_len_bits = msg.len().into_bitarray::<Lsb0>();
    let msg_bits = msg.iter()
        .flat_map(|v| v.into_bitarray::<Lsb0>())
        .collect::<Vec<_>>();
    let payload_bits = msg_len_bits.into_iter()
        .chain(msg_bits.into_iter())
        .collect::<Vec<_>>();

    let img_bytes = img.as_bytes();
    if payload_bits.len() > img_bytes.len() {
        panic!("Cannot embed a payload, the embedding container is too small for the payload");
    }

    let output_buf = img_bytes.iter().enumerate()
        .map(|(i, img_byte)| if i < payload_bits.len() { mask_lsb(img_byte, &payload_bits[i]) } else { *img_byte })
        .collect::<Vec<_>>();

    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
        img.width(),
        img.height(),
        output_buf,
    ).unwrap()
}

fn mask_lsb(img_byte: &u8, payload_bit: &bool) -> u8 {
    if *payload_bit { img_byte | LSB_MASK_TRUE } else { img_byte & LSB_MASK_FALSE }
}
