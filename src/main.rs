use std::env;
use std::fs;
use std::io;

mod merkle_tree;

pub const BLOCK_LENGTH: usize = 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1].clone();
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => panic!("File not valid\n{}", e),
    };

    if metadata.is_file() {
        encrypt(path)?;
    } else if metadata.is_dir() {
        decrypt(path)?;
    } else {
        panic!("Not a file or directory")
    }

    Ok(())
}

fn encrypt(path: &String) -> io::Result<()> {
    println!("Encrypting your file...\n");

    let data = fs::read(path)?;

    let mut blocks: Vec<&[u8]> = Vec::new();
    let mut i = 0;
    while i + BLOCK_LENGTH < data.len() {
        blocks.push(&data[i..i + BLOCK_LENGTH]);
        i += BLOCK_LENGTH;
    }
    blocks.push(&data[i..data.len()]);

    // for (i, block) in blocks.iter().enumerate() {
    //     println!("Block {}:\n{}", i, block);
    // }
    let root = merkle_tree::Node::root(&blocks);
    println!("# Blocks: {}", blocks.len());

    println!("Root:\n{:?}", root);
    println!("xor length:\n{:?}", root.xor.len());

    let key = root.xor;

    for (i, block) in blocks.iter().enumerate() {
        // let encrypted_block = xor(&String::from(*block), &key);
    }
    Ok(())
}

fn decrypt(path: &String) -> io::Result<()> {
    let dir = fs::read_dir(path)?;
    for dir_entry in dir {}
    Ok(())
}

pub fn xor(left_string: &Vec<u8>, right_string: &Vec<u8>) -> Vec<u8> {
    let left = left_string;
    let right = right_string;
    let len = if left.len() < right.len() {
        left.len()
    } else {
        right.len()
    };
    let mut result: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        result.push(left[i] ^ right[i]);
    }
    for i in len..left.len() {
        result.push(left[i]);
    }
    for i in len..right.len() {
        result.push(right[i]);
    }
    Vec::from(result)
}
