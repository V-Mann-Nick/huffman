use bit_vec::BitVec;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn main() {
    let text = "Elit cupiditate sunt earum vel est iure? Quos repudiandae neque placeat ipsa blanditiis. Quidem ad itaque tempore quam culpa Hic ut aliquam quas in voluptas. Eius error eos nemo aspernatur inventore ratione, nam Doloribus voluptas ut numquam consequuntur numquam Tempora vero repellat accusantium quasi dolore culpa dolor? Quis facilis vero.";
    // let text = "My compressed message.";
    huffman_encode(text);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    symbol: Option<char>,
    weight: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn huffman_encode(text: &str) {
    let character_frequencies = character_frequencies(text);
    let mut priority_queue = prioritize(&character_frequencies);
    let root = build_tree(&mut priority_queue);
    println!("Tree:");
    println!("{:#?}", root);
    println!();
    let encoded_tree = encode_huffman_tree(&root);
    println!("Encoded tree:");
    println!("{:?}", encoded_tree);
    println!();
    let codes = codes_from_root(&root);
    println!("Huffman character codes:");
    println!("{:#?}", codes);
    println!();
    let encoded = encode_text(text, &codes);
    println!("Encoded text:");
    println!("{:?}", encoded);
    println!();
    let decoded = decode_text(&encoded, &root);
    println!("Decoded text:");
    println!("{:?}", decoded);
    println!();
}

fn encode_huffman_tree(root: &Node) -> BitVec {
    let mut encoded_tree = BitVec::new();
    encode_huffman_node(root, &mut encoded_tree);
    encoded_tree
}

fn encode_huffman_node(node: &Node, bits: &mut BitVec) {
    if let Some(character) = node.symbol {
        bits.push(true);
        let mut character_bits = BitVec::from_bytes(&[character as u8]);
        bits.append(&mut character_bits);
    } else {
        bits.push(false);
        encode_huffman_node(node.left.as_ref().unwrap(), bits);
        encode_huffman_node(node.right.as_ref().unwrap(), bits);
    }
}

fn encode_text(text: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    let mut decoded = BitVec::new();
    for character in text.chars() {
        let mut code = codes.get(&character).unwrap().clone();
        decoded.append(&mut code);
    }
    decoded
}

fn decode_text(encoded: &BitVec, root: &Node) -> String {
    let mut decoded = String::new();
    let mut current_node = root;
    for bit in encoded.iter() {
        current_node = if bit {
            current_node.right.as_ref().unwrap()
        } else {
            current_node.left.as_ref().unwrap()
        };
        if let Some(character) = current_node.symbol {
            decoded.push(character);
            current_node = root;
        };
    }
    decoded
}

fn codes_from_root(root: &Node) -> HashMap<char, BitVec> {
    let mut code_map: HashMap<char, BitVec> = HashMap::new();
    code_from_node(root, BitVec::new(), &mut code_map);
    code_map
}

fn code_from_node(node: &Node, bits: BitVec, code_map: &mut HashMap<char, BitVec>) {
    match node.symbol {
        Some(symbol) => code_map.insert(symbol, bits),
        None => {
            for (i, branch) in [node.left.as_ref(), node.right.as_ref()].iter().enumerate() {
                let mut bits = bits.clone();
                bits.push(i != 0);
                code_from_node(&branch.unwrap(), bits, code_map);
            }
            None
        }
    };
}

fn build_tree(priority_queue: &mut PriorityQueue<Node, i32>) -> Node {
    while priority_queue.len() > 1 {
        let (first, _) = priority_queue.pop().unwrap();
        let (second, _) = priority_queue.pop().unwrap();
        let weight = first.weight + second.weight;
        let node = Node {
            symbol: None,
            weight,
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
        };
        priority_queue.push(node, -(weight as i32));
    }
    priority_queue.pop().unwrap().0
}

fn prioritize(frequency_map: &HashMap<char, u32>) -> PriorityQueue<Node, i32> {
    let mut pq = PriorityQueue::new();
    for (&character, &frequency) in frequency_map {
        let node = Node {
            symbol: Some(character),
            weight: frequency,
            left: None,
            right: None,
        };
        pq.push(node, -(frequency as i32));
    }
    pq
}

fn character_frequencies(s: &str) -> HashMap<char, u32> {
    let mut frequency_map: HashMap<char, u32> = HashMap::new();
    for character in s.chars() {
        let count = frequency_map.entry(character).or_insert(0);
        *count += 1;
    }
    frequency_map
}
