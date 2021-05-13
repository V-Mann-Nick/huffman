mod tree;
use bit_vec::BitVec;
use tree::{decode, encode};

pub fn huffman_encode(text: &str) -> BitVec {
    let (mut encoded_tree, mut encoded_text) = encode::encode(text);
    let mut bits = BitVec::from_bytes(&(text.len() as u32).to_be_bytes());
    bits.append(&mut encoded_tree);
    bits.append(&mut encoded_text);
    bits
}

pub fn huffman_decode(bits: BitVec) -> String {
    decode::decode(bits)
}
