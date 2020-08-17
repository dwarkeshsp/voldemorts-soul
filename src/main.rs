use regex::Regex;
use std::env;
use std::fs;
use std::io;
mod xor_tree;

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
    let blocks = split(&data);
    let root = xor_tree::Node::make_root(&blocks);
    println!("# Blocks: {}", blocks.len());
    println!("Root:\n{:?}", root);
    println!("xor length: {:?}", root.xor.len());
    let key = root.xor;
    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = xor(&block.to_vec(), &key);
        println!("writing {:?}", encrypted_block);
        let block_name = path.clone() + &"-" + &i.to_string() + &".horcrux";
        fs::write(block_name, encrypted_block)?;
    }
    Ok(())
}

fn split<'a>(data: &'a Vec<u8>) -> Vec<Vec<u8>> {
    let block_size = get_block_size(data.len());
    let mut blocks: Vec<Vec<u8>> = Vec::with_capacity(block_size);
    let mut i = 0;
    while i + block_size < data.len() {
        blocks.push(data[i..i + block_size].to_vec());
        i += block_size;
    }
    let mut remainder: Vec<u8> = Vec::with_capacity(block_size);
    while i < data.len() {
        remainder.push(data[i]);
        i += 1;
    }
    println!("Remainder {:?}", String::from_utf8(remainder.to_vec()));
    blocks.push(remainder.to_vec());
    blocks
}

fn get_block_size(len: usize) -> usize {
    let total_horcruxes = 8;
    let mut block_size = len / total_horcruxes;
    if len % total_horcruxes != 0 {
        block_size += 1;
    }
    println!("Data len {} Block size {}", len, block_size);
    block_size
}
fn decrypt(path: &String) -> io::Result<()> {
    println!("Decrypting {}...\n", path);
    let horcrux_regex = Regex::new(r"(.*?)\.(horcrux)$").unwrap();
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let file_name = file.file_name();
        if horcrux_regex.is_match(file_name.to_str().unwrap()) {
            println!("Reading {:?}", file_name);
            let data = fs::read(file.path()).unwrap();
            println!("{:?}", data);
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
    let mut xor: Vec<u8> = Vec::new();
    for i in 0..len {
        xor.push(left[i] ^ right[i]);
    }
    // if one side is larger, just append the rest of that side
    for i in len..left.len() {
        xor.push(left[i]);
    }
    for i in len..right.len() {
        xor.push(right[i]);
    }
    xor
}
