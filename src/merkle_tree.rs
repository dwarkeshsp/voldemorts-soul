use blake3;

type TreeNode = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    left: TreeNode,
    right: TreeNode,
    hash: [u8; 32],
}

impl Node {
    pub fn new(left: TreeNode, right: TreeNode, block: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(block.as_bytes());
        let mut output = [0; 32];
        let mut output_reader = hasher.finalize_xof();
        output_reader.fill(&mut output);
        Node {
            left: left,
            right: right,
            hash: output,
        }
    }
}

pub fn make_tree(blocks: Vec<&str>) {
    let mut leaves: Vec<Node> = Vec::new();
    for block in blocks {
        leaves.push(Node::new(None, None, block));
    }
}
