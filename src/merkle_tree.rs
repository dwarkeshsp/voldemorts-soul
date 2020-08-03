use blake3;

// type TreeNode = Option<Box<Node>>;
#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    left: Option<NodeId>,
    right: Option<NodeId>,
    hash: [u8; 32],
}

#[derive(Debug, Clone)]
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
        let mut hasher = blake3::Hasher::new();
        hasher.update(block.as_bytes());
        let mut output = [0; 32];
        let mut output_reader = hasher.finalize_xof();
        output_reader.fill(&mut output);
        Node {
            left: None,
            right: None,
            hash: output,
        }
    }

    fn make_from_children(nodes: &Vec<Node>, left: usize, right: usize) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&nodes[left].hash);
        hasher.update(&nodes[left].hash);
        let mut output = [0; 32];
        let mut output_reader = hasher.finalize_xof();
        output_reader.fill(&mut output);
        Node {
            left: Some(NodeId { index: left }),
            right: Some(NodeId { index: right }),
            hash: output,
        }
    }

    fn make_from_child(nodes: &Vec<Node>, left: usize) -> Self {
        let hash = nodes[left].hash;
        Node {
            left: Some(NodeId { index: left }),
            right: None,
            hash: hash,
        }
    }
}
