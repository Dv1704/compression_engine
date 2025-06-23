use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::{File, metadata};
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: <program> <source> <target>");
        return;
    }

    let mut args = args();
    args.next(); // skip program name

    let input_path = args.next().unwrap();
    let output_path = args.next().unwrap();

    // Get source file size before compression
    let source_size = metadata(&input_path).unwrap().len();

    let mut input = BufReader::new(File::open(&input_path).unwrap());
    let output_file = File::create(&output_path).unwrap();
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let _ = encoder.finish().unwrap();

    let duration = start.elapsed();

    // Get target file size after compression
    let target_size = metadata(&output_path).unwrap().len();

    // Calculate compression ratio
    let ratio = if source_size == 0 {
        0.0
    } else {
        (target_size as f64 / source_size as f64) * 100.0
    };

    // Print metrics
    println!("✅ Compression complete");
    println!("🗂️  Source file: {} bytes", source_size);
    println!("📦 Compressed file: {} bytes", target_size);
    println!("📉 Compression ratio: {:.2}%", ratio);
    println!("⏱️  Time taken: {:?}", duration);
}
