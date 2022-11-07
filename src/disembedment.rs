use bitvec::order::Lsb0;
use bitvec::prelude::{bitvec};
use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;

const WORD_LEN: usize = 64;
const LSB_MASK_TRUE: u8 = 0b0000_0001;

pub fn disembed(img: &DynamicImage) -> Vec<u8> {
    let img_bytes = img.as_bytes();
    let msg_len_slice = &img_bytes[0..WORD_LEN];
    let mut msg_len_bits = bitvec!(u8, Lsb0; 0; WORD_LEN);
    for (i, img_byte) in msg_len_slice.iter().enumerate() {
        msg_len_bits.set(i, get_lsb(img_byte));
    }

    let msg_len = msg_len_bits.as_raw_slice().read_u64::<LittleEndian>().unwrap() as usize * 8;
    let mut msg_bits = bitvec!(u8, Lsb0; 0; msg_len);
    let message_slice = &img_bytes[WORD_LEN..WORD_LEN + msg_len];
    for (i, img_byte) in message_slice.iter().enumerate() {
        msg_bits.set(i, get_lsb(img_byte));
    }

    msg_bits.as_raw_slice().to_vec()
}

fn get_lsb(img_byte: &u8) -> bool {
    img_byte & LSB_MASK_TRUE != 0
}
