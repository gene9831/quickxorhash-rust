use base64::{Engine as _, engine::general_purpose};
use quickxorhash::QuickXorHash;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

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
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    // Use BufReader for better I/O performance
    let mut reader = BufReader::new(file);

    // Initialize QuickXorHash
    let mut qx = QuickXorHash::new();

    // Read file in chunks and update hash
    // Use 64KB buffer (160 * 400 = 64000), which is a multiple of BLOCK_SIZE for optimal performance
    let mut buffer = vec![0u8; 160 * 400]; // 64KB buffer (multiple of BLOCK_SIZE)
    loop {
        match reader.read(&mut buffer) {
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
