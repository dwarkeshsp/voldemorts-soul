use crate::xor;

#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub xor: Vec<u8>,
}

impl Node {
    pub fn root(blocks: &Vec<&[u8]>) -> Self {
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

        tree.nodes[tree.nodes.len() - 1].clone()
    }

    fn make_block(block: &[u8]) -> Self {
        Node {
            xor: Vec::from(block),
        }
    }

    fn make_from_children(nodes: &Vec<Node>, left: usize, right: usize) -> Self {
        let xor = xor(&nodes[left].xor, &nodes[right].xor);
        Node { xor: xor }
    }

    fn make_from_child(nodes: &Vec<Node>, left: usize) -> Self {
        Node {
            xor: nodes[left].xor.clone(),
        }
    }
}
