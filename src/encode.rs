use super::{node::Node, progress::Progress, spinner::ProgressSpinner};
use bit_vec::BitVec;
use priority_queue::PriorityQueue;
use std::{iter::FromIterator, path::PathBuf};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
};

pub struct Encoder {
    text: String,
    verbose_level: u8,
    output_file: File,
}

impl Encoder {
    pub fn from_file(
        input_path: &PathBuf,
        output_path: &PathBuf,
        verbose_level: u8,
    ) -> Result<Self, io::Error> {
        let mut input_file = File::open(input_path)?;
        let mut text = String::new();
        input_file.read_to_string(&mut text)?;
        let output_file = File::create(output_path)?;
        Ok(Self {
            text,
            verbose_level,
            output_file,
        })
    }

    pub fn encode(&mut self) -> Result<(), io::Error> {
        let character_frequencies = self.character_frequencies();
        let root = self.build_tree(&character_frequencies);
        let character_codes = self.codes_from_root(&root);
        let serialized_text = self.serialize_text(&character_codes);
        let serialized_tree = self.serialize_tree(&root);
        self.write_to_file(serialized_text, serialized_tree)?;
        Ok(())
    }

    fn write_to_file(
        &mut self,
        mut serialized_text: BitVec,
        mut serialized_tree: BitVec,
    ) -> Result<(), io::Error> {
        let spinner =
            ProgressSpinner::with_title("Writing compressed text to file...", self.verbose_level);
        let mut bits = BitVec::from_bytes(&(self.text.chars().count() as u32).to_be_bytes());
        bits.append(&mut serialized_tree);
        bits.append(&mut serialized_text);
        self.output_file.write(&bits.to_bytes())?;
        spinner.complete_with("Done.");
        Ok(())
    }

    fn codes_from_root(&self, root: &Node) -> HashMap<char, BitVec> {
        let mut code_map: HashMap<char, BitVec> = HashMap::new();
        &self.codes_from_node(root, BitVec::new(), &mut code_map);
        code_map
    }

    fn codes_from_node(&self, node: &Node, bits: BitVec, code_map: &mut HashMap<char, BitVec>) {
        if let Some(symbol) = node.symbol {
            code_map.insert(symbol, bits);
        } else {
            for (i, branch) in [node.left.as_ref(), node.right.as_ref()].iter().enumerate() {
                let mut bits = bits.clone();
                bits.push(i != 0);
                &self.codes_from_node(&branch.unwrap(), bits, code_map);
            }
        }
    }

    fn serialize_text(&self, codes: &HashMap<char, BitVec>) -> BitVec {
        let mut progress_bar = Progress::with_title(
            "Huffman encoding text...",
            self.verbose_level,
            self.text.chars().count(),
        );
        let result =
            self.text
                .chars()
                .enumerate()
                .fold(BitVec::new(), |mut decoded, (i, character)| {
                    progress_bar.set_progress(&i);
                    let mut code = codes.get(&character).unwrap().clone();
                    decoded.append(&mut code);
                    decoded
                });
        progress_bar.complete();
        result
    }

    fn build_tree(&self, frequency_map: &HashMap<char, u32>) -> Node {
        let mut spinner =
            ProgressSpinner::with_title("Building huffman tree...", self.verbose_level);
        let mut priority_queue = self.prioritize(frequency_map);
        while priority_queue.len() >= 2 {
            spinner.bump();
            let (first, priority_first) = priority_queue.pop().unwrap();
            let (second, priority_second) = priority_queue.pop().unwrap();
            let node = Node::internal_node(first, second);
            let priority = priority_first + priority_second;
            priority_queue.push(node, priority as i64);
        }
        spinner.complete_with("Done.");
        let root = priority_queue.pop().unwrap().0;
        self.log(2, format!("{:#?}", root).as_str());
        root
    }

    fn prioritize(&self, frequency_map: &HashMap<char, u32>) -> PriorityQueue<Node, i64> {
        PriorityQueue::from_iter(
            frequency_map
                .iter()
                .map(|(character, frequency)| (Node::leaf_node(*character), -(*frequency as i64))),
        )
    }

    fn character_frequencies(&self) -> HashMap<char, u32> {
        let mut progress_bar = Progress::with_title(
            "Computing character frequencies...",
            self.verbose_level,
            self.text.chars().count(),
        );
        let result = self.text.chars().enumerate().fold(
            HashMap::new(),
            |mut character_frequencies, (i, character)| {
                progress_bar.set_progress(&i);
                let count = character_frequencies.entry(character).or_insert(0);
                *count += 1;
                character_frequencies
            },
        );
        self.log(2, format!("{:#?}", result).as_str());
        progress_bar.complete();
        result
    }

    fn serialize_tree(&self, root: &Node) -> BitVec {
        let mut spinner =
            ProgressSpinner::with_title("Serializing huffman tree...", self.verbose_level);
        let mut encoded_tree = BitVec::new();
        self.serialize_node(root, &mut encoded_tree, &mut spinner);
        spinner.complete_with("Done.");
        encoded_tree
    }

    fn serialize_node(&self, node: &Node, bits: &mut BitVec, spinner: &mut ProgressSpinner) {
        if let Some(character) = node.symbol {
            spinner.bump();
            bits.push(true);
            let mut buffer = [0; 4];
            character.encode_utf8(&mut buffer);
            let mut character_bits = BitVec::from_bytes(&buffer[0..character.len_utf8()]);
            bits.append(&mut character_bits);
        } else {
            bits.push(false);
            self.serialize_node(node.left.as_ref().unwrap(), bits, spinner);
            self.serialize_node(node.right.as_ref().unwrap(), bits, spinner);
        }
    }

    fn log(&self, level: u8, message: &str) {
        if self.verbose_level >= level {
            println!("{}", message);
        }
    }
}
