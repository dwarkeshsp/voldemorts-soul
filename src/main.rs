use std::env;
use std::fs;

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
    println!("Encrypting your file:\n{}", data);
}

fn decrypt(dir: fs::ReadDir) {}
