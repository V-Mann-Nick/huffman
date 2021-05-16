use super::{node::Node, progress::Progress, spinner::ProgressSpinner};
use endio_bit::BEBitReader;
use num_format::{Locale, ToFormattedString};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    str::from_utf8,
};

pub struct Decoder {
    verbose_level: u8,
    input_path: PathBuf,
    output_path: PathBuf,
}

impl Decoder {
    pub fn from_file(input_path: PathBuf, output_path: PathBuf, verbose_level: u8) -> Self {
        Self {
            input_path,
            output_path,
            verbose_level,
        }
    }

    pub fn decode(&self) -> Result<(), io::Error> {
        let file = File::open(&self.input_path)?;
        let mut reader = BEBitReader::new(file);
        let text_length = self.parse_header(&mut reader)?;
        let tree = self.parse_tree(&mut reader)?;
        self.decode_text(&mut reader, text_length, &tree)?;
        Ok(())
    }

    fn parse_header(&self, file: &mut BEBitReader<File>) -> Result<u32, io::Error> {
        let mut buffer = [0; 4];
        file.take(4).read(&mut buffer)?;
        let result = u32::from_be_bytes(buffer);
        self.log(
            1,
            format!(
                "Number of characters: {}",
                result.to_formatted_string(&Locale::en)
            ).as_str(),
        );
        Ok(result)
    }

    fn parse_tree(&self, bit_reader: &mut BEBitReader<File>) -> Result<Node, io::Error> {
        let mut spinner = ProgressSpinner::with_title("De-serializing tree...", self.verbose_level);
        let root = self.parse_node(bit_reader, &mut spinner)?;
        spinner.complete_with("Done.");
        Ok(root)
    }

    fn parse_node(
        &self,
        bit_reader: &mut BEBitReader<File>,
        spinner: &mut ProgressSpinner,
    ) -> Result<Node, io::Error> {
        spinner.bump();
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
            let left = self.parse_node(bit_reader, spinner)?;
            let right = self.parse_node(bit_reader, spinner)?;
            Ok(Node::internal_node(left, right))
        }
    }

    fn decode_text(
        &self,
        bit_reader: &mut BEBitReader<File>,
        text_length: u32,
        root: &Node,
    ) -> Result<(), io::Error> {
        let mut progress_bar = Progress::with_title(
            "De-serializing the text...",
            self.verbose_level,
            text_length as usize,
        );
        let mut output_file = File::create(&self.output_path)?;
        const BATCH_SIZE: u32 = 1_000_000;
        let mut decoded_batch = String::new();
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
                progress_bar.set_progress(&((text_length - countdown) as usize));
                decoded_batch.push(character);
                current_node = root;
                countdown -= 1;
                if countdown % BATCH_SIZE == 0 {
                    output_file.write(decoded_batch.as_bytes())?;
                    decoded_batch.clear();
                }
                if countdown == 0 {
                    break;
                }
            };
        }
        output_file.write(decoded_batch.as_bytes())?;
        progress_bar.complete();
        Ok(())
    }

    fn log(&self, level: u8, message: &str) {
        if self.verbose_level >= level {
            println!("{}", message)
        }
    }
}
