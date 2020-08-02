use std::env;
use std::fs;

mod merkle_tree;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1].clone();
    let is_file = fs::metadata(path)?.is_file();

    if is_file {
        encrypt(fs::read_to_string(path)?);
    } else {
        decrypt(fs::read_dir(path)?);
    }

    Ok(())
}

fn encrypt(data: String) {
    const BLOCK_LENGTH: usize = 512;

    //println!("Encrypting your file:\n{}\n", data);

    let mut blocks: Vec<&str> = Vec::new();
    let mut i = 0;
    while i + BLOCK_LENGTH < data.len() {
        blocks.push(&data[i..i + BLOCK_LENGTH]);
        i += BLOCK_LENGTH;
    }
    blocks.push(&data[i..data.len()]);

    // for (i, block) in blocks.iter().enumerate() {
    //     println!("Block {}:\n{}", i, block);
    // }
    let node = merkle_tree::Node::make_tree(blocks);
    println!("{:?}", node)
}

fn decrypt(dir: fs::ReadDir) {}
