use super::node::Node;
use endio_bit::BEBitReader;
use std::{
    error,
    fs::File,
    io::{self, Read},
    str::from_utf8,
};
use termprogress::prelude::*;

pub fn decode(file: File) -> Result<String, Box<dyn error::Error>> {
    let mut reader = BEBitReader::new(file);
    let text_length = parse_header(&mut reader)?;
    let tree = parse_tree(&mut reader)?;
    Ok(decode_text(&mut reader, text_length, &tree)?)
}

fn parse_header(file: &mut BEBitReader<File>) -> Result<u32, io::Error> {
    let mut buffer = [0; 4];
    file.take(4).read(&mut buffer)?;
    Ok(u32::from_be_bytes(buffer))
}

fn parse_tree(bit_reader: &mut BEBitReader<File>) -> Result<Node, io::Error> {
    let bit = bit_reader.read_bit()?;
    if bit {
        let mut buffer = [0; 4];
        for i in 0..4 {
            bit_reader.take(1).read(&mut buffer[i..(i + 1)])?;
            if let Ok(character) = from_utf8(&buffer[0..(i + 1)]) {
                return Ok(Node::leaf_node(character.chars().nth(0).unwrap()));
            }
        }
        panic!("character couldn't be parsed as unicode")
    } else {
        let left = parse_tree(bit_reader)?;
        let right = parse_tree(bit_reader)?;
        Ok(Node::internal_node(left, right))
    }
}

fn decode_text(
    bit_reader: &mut BEBitReader<File>,
    text_length: u32,
    root: &Node,
) -> Result<String, io::Error> {
    let mut progress_bar = Bar::default();
    progress_bar.set_title("Decoding...");
    let num_characters_per_progress_update = text_length / 100;
    let mut decoded = String::new();
    let mut current_node = root;
    let mut countdown = text_length;
    loop {
        let bit = bit_reader.read_bit()?;
        current_node = if bit {
            current_node.right.as_ref().unwrap()
        } else {
            current_node.left.as_ref().unwrap()
        };
        if let Some(character) = current_node.symbol {
            decoded.push(character);
            current_node = root;
            countdown -= 1;
            if countdown % num_characters_per_progress_update == 0 {
                let progress = (text_length - countdown) as f64 / text_length as f64;
                progress_bar.set_progress(progress);
            }
            if countdown == 0 {
                break;
            }
        };
    }
    Ok(decoded)
}
