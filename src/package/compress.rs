use log::{debug, error, info};
use lz4::EncoderBuilder;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
    sync::atomic::AtomicBool,
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
        error!("error occurred, {:?}", err);
    }

    return out;
}

use crate::util::error::{self, ErrorKind, PlaygroundError};
// not parallel
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
                    debug!("use compressed, ratio:{}", ratio);
                    result.insert(count, compressed);
                } else {
                    debug!("not use compressed, ratio:{}", ratio);
                    result.insert(count, buf.to_vec());
                }
            }
        }
        count += 1;
    }
}

// TODO: add error handle.
fn _rayon_sample() {
    debug!("rayon sample");
    let file = File::open(FILENAME).unwrap();
    let metadata = file.metadata().unwrap();
    let mut reader = BufReader::new(file);
    let mut buf;

    let mut result = Vec::new();
    let mut src_offset = 0;
    let mut cmp_offset = 0;
    let mut comp_table = Vec::with_capacity(1);
    let is_compressed: AtomicBool = AtomicBool::new(false);

    loop {
        buf = vec![0; BLOCK_SIZE * 10];
        match reader.read(&mut buf).unwrap() {
            0 => {
                info!("finish reading");
                break;
            }
            n => {
                // if last block is less than BLOCK_SIZE, already 0-filled
                let mut data: Vec<(usize, Vec<u8>, usize)> = buf
                    .chunks(BLOCK_SIZE)
                    .enumerate()
                    .par_bridge()
                    .map(|(i, data)| {
                        // println!("count:{}", i);
                        let compressed_data = compress(&data);
                        let ratio = (compressed_data.len() as f64 / data.len() as f64) * 100.0;
                        if ratio <= THRESHOLD {
                            debug!("use compressed, ratio:{}", ratio);
                            // if compressed set flag
                            is_compressed.store(true, std::sync::atomic::Ordering::SeqCst);
                            (i, compressed_data, data.len())
                        } else {
                            debug!("not use compressed, ratio:{}", ratio);
                            // TODO: it is better that it uses buf memory
                            (i, data.to_vec(), data.len())
                        }
                    })
                    .collect();
                // result is not in order
                data.sort_by_key(|d| d.0);

                // create compression table (src_size,src_offset,cmp_size,cmp_offset) and move result
                for d in &mut data {
                    comp_table.push((d.2, src_offset, d.1.len(), cmp_offset));
                    cmp_offset += d.1.len();
                    src_offset += d.2;

                    // move values
                    result.append(&mut d.1)
                }
            }
        }
    }

    let is_compressed_result = is_compressed.load(std::sync::atomic::Ordering::SeqCst);
    info!("is_compressed:{} ", is_compressed_result);
    // need to remove
    if !is_compressed_result {
        debug!(
            "all blocks are not compressed, so need to remove additional buffer from last block"
        );
        if (metadata.len() as usize) != result.len() {
            panic!(
                "read data length and file size is not matched. read data size:{}, file size:{}",
                metadata.len(),
                result.len()
            );
        }
        // make read data to original size
        result.truncate(metadata.len() as usize);
    }

    debug!("compression table:{:?}", comp_table);
    info!("finish compression length:{} byte", result.len());
}

pub fn parallel_compress() {
    _rayon_sample();
}

// TODO: add test
