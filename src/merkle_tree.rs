use crate::BLOCK_LENGTH;

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    left: Option<NodeId>,
    right: Option<NodeId>,
    xor: String,
}

#[derive(Debug)]
pub struct NodeId {
    index: usize,
}

impl Tree {
    pub fn new(blocks: Vec<&str>) -> Self {
        let mut tree = Tree { nodes: Vec::new() };
        for block in blocks {
            tree.nodes.push(Node::make_block(block));
        }

        let mut cursor = 0;
        let mut len = tree.nodes.len();
        while cursor < len - 1 {
            for index in cursor..len {
                if index % 2 == 0 {
                    if index + 1 < len {
                        //matched; has sibling
                        tree.nodes
                            .push(Node::make_from_children(&tree.nodes, index, index + 1));
                    } else {
                        tree.nodes.push(Node::make_from_child(&tree.nodes, index));
                    }
                }
            }
            cursor = len;
            len = tree.nodes.len();
        }

        tree
    }
}

impl Node {
    fn make_block(block: &str) -> Self {
        Node {
            left: None,
            right: None,
            xor: String::from(block),
        }
    }

    fn make_from_children(nodes: &Vec<Node>, left: usize, right: usize) -> Self {
        let xor = xor(&nodes[left].xor.as_bytes(), &nodes[right].xor.as_bytes());
        Node {
            left: Some(NodeId { index: left }),
            right: Some(NodeId { index: right }),
            xor: xor,
        }
    }

    fn make_from_child(nodes: &Vec<Node>, left: usize) -> Self {
        let node = &nodes[left];
        Node {
            left: Some(NodeId { index: left }),
            right: None,
            xor: node.xor.clone(),
        }
    }
}

pub fn xor(left: &[u8], right: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::with_capacity(BLOCK_LENGTH);
    for i in 0..right.len() {
        result.push(left[i] ^ right[i]);
    }
    String::from_utf8(result).unwrap()
}
