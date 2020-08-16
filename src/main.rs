use regex::Regex;
use std::env;
use std::fs;
use std::io;

mod xor_tree;

pub const BLOCK_LENGTH: usize = 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1].clone();

    let metadata = get_metadata(path);
    if metadata.is_file() {
        encrypt(path)?;
    } else if metadata.is_dir() {
        decrypt(path)?;
    } else {
        panic!("Not a file or directory")
    }

    Ok(())
}

fn get_metadata(path: &String) -> fs::Metadata {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => panic!("File not valid\n{}", e),
    };
    metadata
}

fn encrypt(path: &String) -> io::Result<()> {
    println!("Encrypting {}...\n", path);

    let data = fs::read(path)?;
    let mut remainder: [u8; BLOCK_LENGTH] = [0; BLOCK_LENGTH];

    let blocks = split(&data, &mut remainder);

    let root = xor_tree::Node::make_root(&blocks);
    println!("# Blocks: {}", blocks.len());
    println!("Root:\n{:?}", root);
    println!("xor length: {:?}", root.xor.len());

    let key = root.xor;
    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = xor(&block.to_vec(), &key);

        let block_name = path.clone() + &"-" + &i.to_string() + &".horcrux";
        fs::write(block_name, encrypted_block)?;
    }
    Ok(())
}

fn split<'a>(data: &'a Vec<u8>, remainder: &'a mut [u8; BLOCK_LENGTH]) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    while i + BLOCK_LENGTH < data.len() {
        blocks.push(data[i..i + BLOCK_LENGTH].to_vec());
        i += BLOCK_LENGTH;
    }
    while i < data.len() {
        remainder[i % BLOCK_LENGTH] = data[i];
        i += 1;
    }
    println!("{:?}", String::from_utf8(remainder.to_vec()));
    blocks.push(remainder.to_vec());
    blocks
}

fn decrypt(path: &String) -> io::Result<()> {
    println!("Decrypting {}...\n", path);

    let horcrux_regex = Regex::new(r"(.*?)\.(horcrux)$").unwrap();

    let mut blocks: Vec<Vec<u8>> = Vec::new();

    for entry in fs::read_dir(path)? {
        let file = entry?;
        let file_name = file.file_name();
        if horcrux_regex.is_match(file_name.to_str().unwrap()) {
            println!("{:?}", file_name);
            let data = fs::read(file.path()).unwrap();
            blocks.push(data);
        }
    }

    let root = xor_tree::Node::make_root(&blocks);
    println!("Root:\n{:?}", root);
    println!("# Blocks: {}", blocks.len());

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
    // if one side is larger, just append the rest of that side
    for i in len..left.len() {
        result.push(left[i]);
    }
    for i in len..right.len() {
        result.push(right[i]);
    }
    result
}
