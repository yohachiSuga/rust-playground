use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
};

use byteorder::WriteBytesExt;
use lz4::EncoderBuilder;

const BLOCK_SIZE: usize = 64 * 1024;
const THRESHOLD: f64 = 90.0;

// TODO: use not heap data?
fn compress(buf: &[u8]) -> Vec<u8> {
    let mut encoder = EncoderBuilder::new()
        .level(4)
        .build(Vec::with_capacity(BLOCK_SIZE + 1000))
        .unwrap();
    encoder.write(buf).unwrap();
    let (out, result) = encoder.finish();
    if let Err(err) = result {
        println!("error occurred, {:?}", err);
    }

    println!(
        "compressed:{:?} len:{}, cap:{}",
        &out[0..10],
        out.len(),
        out.capacity()
    );
    return out;
}

// TODO: use thread pool
pub fn compress_sample() {
    let mut reader = BufReader::new(File::open("data").unwrap());
    let mut buf;

    // TODO: not use hashmap is better? hashmap size is already known by using FileSize/BLOCK_SIZE
    // it might be too large on stack.
    let mut result = HashMap::new();
    let mut count: usize = 0;
    loop {
        buf = [0; BLOCK_SIZE];
        match reader.read(&mut buf).unwrap() {
            0 => break,
            n => {
                // println!("read size: {}", n);
                let compressed = compress(&buf);
                // good for compress
                let ratio = (compressed.len() as f64 / n as f64) * 100.0;
                if ratio <= THRESHOLD {
                    println!("use compressed, ratio:{}", ratio);
                    result.insert(count, compressed);
                } else {
                    println!("not use compressed, ratio:{}", ratio);
                    result.insert(count, buf.to_vec());
                }
            }
        }
        count += 1;
    }
}
