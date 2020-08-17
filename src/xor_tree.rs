use crate::xor;

pub struct Tree {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub xor: Vec<u8>,
}

pub fn get_root_xor(blocks: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut tree = Tree { nodes: Vec::new() };
    let nodes = &mut tree.nodes;
    for block in blocks {
        nodes.push(Node {
            xor: block.to_vec(),
        });
    }
    let mut cursor = 0;
    let mut len = nodes.len();
    while cursor < len - 1 {
        for index in cursor..len {
            if index % 2 == 0 {
                if index + 1 < len {
                    //matched; parent is xor of siblings
                    let xor = xor(&nodes[index].xor, &nodes[index + 1].xor);
                    nodes.push(Node { xor: xor });
                } else {
                    // unmatched (remainder or its parent); make parent clone of child
                    // should not be necessary with 8 (power of 2) horcruxes
                    let xor = nodes[index].xor.clone();
                    nodes.push(Node { xor: xor });
                }
            }
        }
        cursor = len;
        len = nodes.len();
    }
    nodes[nodes.len() - 1].xor.clone()
}
