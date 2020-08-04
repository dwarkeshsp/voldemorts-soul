use rand::{thread_rng, Rng};
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
    println!("Encrypting {}...\n", path);

    let data = fs::read(path)?;
    let mut remainder: [u8; BLOCK_LENGTH] = [0; BLOCK_LENGTH];

    let blocks = split(&data, &mut remainder);

    let root = merkle_tree::Node::root(&blocks);
    println!("# Blocks: {}", blocks.len());
    println!("Root:\n{:?}", root);
    println!("xor length: {:?}", root.xor.len());

    let key = root.xor;

    for (i, block) in blocks.iter().enumerate() {
        // let encrypted_block = xor(&String::from(*block), &key);
    }
    Ok(())
}

fn split<'a>(data: &'a Vec<u8>, remainder: &'a mut [u8; BLOCK_LENGTH]) -> Vec<&'a [u8]> {
    let mut blocks: Vec<&[u8]> = Vec::new();
    let mut i = 0;
    while i + BLOCK_LENGTH < data.len() {
        blocks.push(&data[i..i + BLOCK_LENGTH]);
        i += BLOCK_LENGTH;
    }
    for i in 0..i % BLOCK_LENGTH {
        remainder[i] = data[i];
    }
    // thread_rng().fill(&mut *remainder);
    blocks.push(remainder);
    blocks
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
    let mut result: Vec<u8> = Vec::new();

    for i in 0..len {
        result.push(left[i] ^ right[i]);
    }
    for i in len..left.len() {
        result.push(left[i]);
    }
    for i in len..right.len() {
        result.push(right[i]);
    }
    result
}
