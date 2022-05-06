use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
};

use byteorder::WriteBytesExt;
use lz4::EncoderBuilder;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    vec,
};

const BLOCK_SIZE: usize = 64 * 1024;
const THRESHOLD: f64 = 90.0;
// const FILENAME: &str = "data";
const FILENAME: &str = "binary";

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
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
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
                // println!("read size: {}", n;
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

fn _rayon_sample() {
    println!("rayon sample");
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut buf;
    loop {
        buf = vec![0; BLOCK_SIZE * 10];
        match reader.read(&mut buf).unwrap() {
            0 => {
                println!("read but 0 return");
                break;
            }
            n => {
                // remove additional buf
                buf.truncate(n);
                let mut data: Vec<(usize, Vec<u8>)> = buf
                    .chunks(BLOCK_SIZE)
                    .enumerate()
                    .par_bridge()
                    .map(|(i, data)| {
                        // println!("count:{}", i);
                        (i, compress(&data))
                    })
                    .collect();
                // result is not in order
                data.sort_by_key(|d| d.0);
            }
        }
    }
}

pub fn parallel_compress() {
    _rayon_sample();
}
