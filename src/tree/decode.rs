use super::node::Node;
use bit_vec::BitVec;
use std::convert::TryInto;

pub fn decode(bits: BitVec) -> String {
    let mut offset = 0 as usize;
    let text_length = parse_header(&bits, &mut offset);
    let tree = parse_tree(&bits, &mut offset);
    decode_text(&bits, &mut offset, text_length, &tree)
}

fn parse_header(bits: &BitVec, offset: &mut usize) -> u32 {
    let mut text_length = BitVec::new();
    while *offset < 32 {
        text_length.push(bits.get(*offset).unwrap());
        *offset += 1;
    }
    u32::from_be_bytes(text_length.to_bytes().try_into().unwrap())
}

fn parse_tree(bits: &BitVec, offset: &mut usize) -> Node {
    let bit = bits.get(*offset).unwrap();
    *offset += 1;
    if bit {
        let mut character = BitVec::new();
        for i in (*offset)..(*offset + 8) {
            character.push(bits.get(i).unwrap())
        }
        *offset += 8;
        let character = character.to_bytes()[0] as char;
        Node::leaf_node(character)
    } else {
        let left = parse_tree(bits, offset);
        let right = parse_tree(bits, offset);
        Node::internal_node(left, right)
    }
}

fn decode_text(bits: &BitVec, offset: &mut usize, text_length: u32, root: &Node) -> String {
    let mut decoded = String::new();
    let mut current_node = root;
    let mut countdown = text_length;
    loop {
        let bit = bits.get(*offset).unwrap();
        *offset += 1;
        current_node = if bit {
            current_node.right.as_ref().unwrap()
        } else {
            current_node.left.as_ref().unwrap()
        };
        if let Some(character) = current_node.symbol {
            decoded.push(character);
            current_node = root;
            countdown -= 1;
            if countdown == 0 {
                break;
            }
        };
    }
    decoded
}
