use bit_vec::BitVec;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::iter::FromIterator;
use super::node::Node;

pub fn encode(text: &str) -> BitVec {
    let character_frequencies = character_frequencies(text);
    let root = build_tree(&character_frequencies);
    let character_codes = codes_from_root(&root);
    let mut encoded_text = encode_text(text, &character_codes);
    let mut encoded_tree = encode_huffman_tree(&root);
    let mut bits = BitVec::from_bytes(&(text.chars().count() as u32).to_be_bytes());
    bits.append(&mut encoded_tree);
    bits.append(&mut encoded_text);
    bits
}

fn codes_from_root(root: &Node) -> HashMap<char, BitVec> {
    let mut code_map: HashMap<char, BitVec> = HashMap::new();
    codes_from_node(root, BitVec::new(), &mut code_map);
    code_map
}

fn codes_from_node(node: &Node, bits: BitVec, code_map: &mut HashMap<char, BitVec>) {
    if let Some(symbol) = node.symbol {
        code_map.insert(symbol, bits);
    } else {
        for (i, branch) in [node.left.as_ref(), node.right.as_ref()].iter().enumerate() {
            let mut bits = bits.clone();
            bits.push(i != 0);
            codes_from_node(&branch.unwrap(), bits, code_map);
        }
    }
}

fn encode_text(text: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    text.chars().fold(BitVec::new(), |mut decoded, character| {
        let mut code = codes.get(&character).unwrap().clone();
        decoded.append(&mut code);
        decoded
    })
}

fn build_tree(frequency_map: &HashMap<char, u32>) -> Node {
    let mut priority_queue = prioritize(frequency_map);
    while priority_queue.len() >= 2 {
        let (first, priority_first) = priority_queue.pop().unwrap();
        let (second, priority_second) = priority_queue.pop().unwrap();
        let node = Node::internal_node(first, second);
        let priority = priority_first + priority_second;
        priority_queue.push(node, priority as i64);
    }
    priority_queue.pop().unwrap().0
}

fn prioritize(frequency_map: &HashMap<char, u32>) -> PriorityQueue<Node, i64> {
    PriorityQueue::from_iter(
        frequency_map
            .iter()
            .map(|(character, frequency)| (Node::leaf_node(*character), -(*frequency as i64))),
    )
}

fn character_frequencies(s: &str) -> HashMap<char, u32> {
    s.chars()
        .fold(HashMap::new(), |mut character_frequencies, character| {
            let count = character_frequencies.entry(character).or_insert(0);
            *count += 1;
            character_frequencies
        })
}

fn encode_huffman_tree(root: &Node) -> BitVec {
    let mut encoded_tree = BitVec::new();
    encode_huffman_node(root, &mut encoded_tree);
    encoded_tree
}

fn encode_huffman_node(node: &Node, bits: &mut BitVec) {
    if let Some(character) = node.symbol {
        bits.push(true);
        let mut buffer = [0; 4];
        character.encode_utf8(&mut buffer);
        let mut character_bits = BitVec::from_bytes(&buffer[0..character.len_utf8()]);
        bits.append(&mut character_bits);
    } else {
        bits.push(false);
        encode_huffman_node(node.left.as_ref().unwrap(), bits);
        encode_huffman_node(node.right.as_ref().unwrap(), bits);
    }
}
