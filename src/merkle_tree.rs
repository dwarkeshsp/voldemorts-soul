use blake3;

type TreeNode = Option<Box<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
    left: TreeNode,
    right: TreeNode,
    hash: [u8; 32],
}

impl Node {
    pub fn make_tree(blocks: Vec<&str>) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        for block in blocks {
            nodes.push(Node::make_from_block(None, None, block));
        }
        recursive_combine_children(nodes)
    }

    fn make_from_block(left: TreeNode, right: TreeNode, block: &str) -> Self {
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

    fn make_from_children(left: Node, right: Node) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let mut output = [0; 32];
        let mut output_reader = hasher.finalize_xof();
        output_reader.fill(&mut output);
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            hash: output,
        }
    }

    fn make_from_child(left: Node) -> Self {
        let hash = left.hash;
        Node {
            left: Some(Box::new(left)),
            right: None,
            hash: hash,
        }
    }
}

fn recursive_combine_children(children: Vec<Node>) -> Node {
    // base condition: root returned
    if children.len() == 1 {
        return children[0].clone();
    }

    let mut parents: Vec<Node> = Vec::new();
    println!("{}", children.len());
    for i in 0..children.len() {
        if i % 2 == 0 {
            // there is a sibling
            if i + 1 < children.len() {
                parents.push(Node::make_from_children(
                    children[i].clone(),
                    children[i + 1].clone(),
                ));
            } else {
                parents.push(Node::make_from_child(children[i].clone()));
            }
        }
    }

    recursive_combine_children(parents)
}
