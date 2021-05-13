#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node {
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn internal_node(left: Self, right: Self) -> Self {
        Self {
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn leaf_node(symbol: char) -> Self {
        Self {
            symbol: Some(symbol),
            left: None,
            right: None,
        }
    }
}
