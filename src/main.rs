use base64::{Engine as _, engine::general_purpose};
use quickxorhash::QuickXorHash;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filePath>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read file content
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    // Initialize QuickXorHash
    let mut qx = QuickXorHash::new();

    // Read file in chunks and update hash
    let mut buffer = vec![0u8; 8192]; // 8KB buffer
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                qx.update(&buffer[..n]);
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    }

    // Finalize hash
    let hash = qx.finalize();

    // Encode to base64 and print
    let base64_hash = general_purpose::STANDARD.encode(&hash);
    println!("{}", base64_hash);
}
