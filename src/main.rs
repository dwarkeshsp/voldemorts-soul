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
    let key = root.xor;
    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = xor(&block.to_vec(), &key);
        let block_name = path.to_string() + &"-encrypted" + &i.to_string() + &".horcrux";
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
    while remainder.len() % block_size != 0 {
        // fill with whitespace
        remainder.push(32);
    }
    blocks.push(remainder.to_vec());
    blocks
}

fn get_block_size(len: usize) -> usize {
    let total_horcruxes = 8;
    let mut block_size = len / total_horcruxes;
    if len % total_horcruxes != 0 {
        block_size += 1;
    }
    block_size
}
fn decrypt(path: &String) -> io::Result<()> {
    println!("Decrypting {}...\n", path);
    let horcrux_regex = Regex::new(r"(.*?)\.(horcrux)$").unwrap();
    let mut file_paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let file_path = file.path();
        let path_string = file_path.to_str().unwrap();
        if horcrux_regex.is_match(&path_string) {
            file_paths.push(path_string.to_string());
        }
    }
    file_paths.sort();
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for file in file_paths.clone() {
        let data = fs::read(file).unwrap();
        blocks.push(data);
    }
    let root = xor_tree::Node::make_root(&blocks);
    for i in 0..blocks.len() {
        blocks[i] = xor(&blocks[i], &root.xor);
    }

    fs::write(
        file_paths[0].split('-').collect::<Vec<&str>>()[0].to_owned() + "-decrypted.horcrux",
        blocks.concat(),
    )?;
    Ok(())
}

pub fn xor(left_string: &Vec<u8>, right_string: &Vec<u8>) -> Vec<u8> {
    let left = left_string;
    let right = right_string;
    let mut xor: Vec<u8> = Vec::new();
    // remainder (possibly shorter than block_size) will always be on the right side
    for i in 0..right.len() {
        xor.push(left[i] ^ right[i]);
    }
    xor
}
