use std::{
    any::TypeId,
    cell::{Cell, RefCell},
    env,
    error::Error,
    fmt::{write, Debug, Result},
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
    thread,
    time::Instant,
    vec,
};

use byteorder::{LittleEndian, WriteBytesExt};
// use my_error::{ErrorKind, PlaygroundError};

// fn _raw_ptr() {
//     // rawpointer test
//     let mut a = 100;
//     println!("{:p}", &a);
//     println!("{:p}", &a as *const i32);
//     let raw_a = &mut a as *mut i32;
//     unsafe {
//         if raw_a.is_null() {
//             println!("nullpointer");
//             return;
//         }
//         *raw_a = 300;
//     }
//     println!("{:p} {}", &a, a);
// }

// // it is better to slice as arguments.
// // because array and vector is acceptable when it is slice.
// // (the same can be said of String and &str)
// // slice is fat pointer with length and pointer to start pos.
// fn sort<T>(v: &mut [T])
// where
//     T: std::cmp::Eq + std::cmp::Ord + Debug + Copy,
// {
//     for i in 0..v.len() {
//         for j in i..v.len() {
//             if v[j] < v[i] {
//                 let tmp = v[j];
//                 v[j] = v[i];
//                 v[i] = tmp;
//             }
//         }
//     }
// }

// fn _slice() {
//     let mut a: [isize; 4] = [2, 1, 2, 3];
//     let mut v = vec![2, 1, 3, 0];
//     sort(&mut v);
//     sort(&mut a);
//     println!("{:?}", v);
//     println!("{:?}", a);
// }

// fn thread() {
//     let mut handles = Vec::with_capacity(10);
//     for i in 0..10 {
//         handles.push(thread::spawn(move || {
//             if i % 2 == 0 {
//                 panic!("panic here!");
//             }
//             println!("{}", i);
//         }));
//     }

//     for h in handles {
//         match h.join() {
//             Ok(()) => {
//                 println!("not panic")
//             }
//             _ => {
//                 println!("panic")
//             }
//         }
//     }
// }

// fn _movetest() {
//     // for loop move v to inside for loop, so error occurred
//     // let v = vec!["a".to_string(), "b".to_string()];
//     // for mut s in v {
//     //     s.push('!');
//     // }
//     // println!("{:?}", v);
//     // below is OK because of using reference.
//     let v = vec!["a".to_string(), "b".to_string()];
//     for s in &v {
//         let a = 1;
//     }
//     println!("{:?}", v);

//     #[derive(Debug, Clone, Copy)]
//     struct EmbedObj {
//         j: i32,
//     }
//     #[derive(Debug, Clone, Copy)]
//     struct CopyObj {
//         i: i32,
//         obj: EmbedObj,
//     }
//     let i = 100;
//     let e = EmbedObj { j: i + 1 };
//     let o = CopyObj { i: i + 10, obj: e };
//     let o2 = o;
//     println!("{:?} {:?}", o, o2);
// }

// fn _reference() {
//     struct RefObj {
//         num: u32,
//     }
//     let o1 = RefObj { num: 100 };
//     let ref_obj = &o1;
//     assert_eq!(ref_obj.num, (*ref_obj).num);
// }

// #[repr(u8)]
// #[derive(Debug)]
// enum TypeName {
//     PackageName = 0x01,
//     Flow = 0x02,
// }

// // use
// #[derive(Debug)]
// struct TLV {
//     t: TypeName,
//     l: usize,
//     v: Vec<u8>,
// }

// struct PackageName {
//     block: TLV,
// }
// struct Flow {
//     block: TLV,
// }

// trait TLVBase {
//     fn block(&self) -> &TLV;
//     fn dump(&mut self) -> &mut [u8];
// }

// impl PackageName {
//     fn new(name: String) -> PackageName {
//         PackageName {
//             block: TLV {
//                 t: TypeName::PackageName,
//                 l: name.len(),
//                 v: name.into_bytes(),
//             },
//         }
//     }
// }

// impl TLVBase for PackageName {
//     fn block(&self) -> &TLV {
//         &self.block
//     }

//     fn dump(&mut self) -> &mut [u8] {
//         &mut self.block.v
//     }
// }

// impl Flow {
//     fn new() -> Flow {
//         Flow {
//             block: TLV {
//                 t: TypeName::Flow,
//                 l: 1,
//                 v: vec![1],
//             },
//         }
//     }
// }

// impl TLVBase for Flow {
//     fn block(&self) -> &TLV {
//         &self.block
//     }

//     fn dump(&mut self) -> &mut [u8] {
//         &mut self.block.v
//     }
// }

// /**
// endian
// 1byteより大きなバイト列の格納順序を指定する。
// つまり、ある処理したいパラメータのサイズが1byteであれば、それはEndianの影響を受けない。

// 文字列は？？
// =>文字コードに依存する. ASCIIであれば全ての文字が1byteなので気にしなくて良い
//     UTF-8の場合最大4バイトなので、byteorderが関係してくる。 TODO: pythonのコードを確

//  */
// impl TLV {
//     // TODO: HOW to dump tlv value with endian??
//     fn dump(&self) -> Vec<u8> {
//         let mut wtr = Vec::with_capacity(self.l as usize);
//         // wtr.write_u8::<LittleEndian>(self.t).unwrap();
//         // wtr.write_u16::<LittleEndian>(self.l).unwrap();
//         // wtr.write::<LittleEndian>(&self.v).unwrap();
//         // wtr.write_all::<LittleEndian>(&self.v).unwrap();
//         wtr
//     }
// }

// fn exp_aup() {
//     let mut writer = BufWriter::new(File::create("./out.dump").unwrap());

//     let mut p_name = PackageName::new("package name".to_string());
//     writer.write_all(p_name.dump()).unwrap();

//     let mut script = Flow::new();
//     writer.write_all(script.dump()).unwrap();

//     writer.flush();
// }

// fn _looptest() -> usize {
//     fn gen_num() -> u64 {
//         4
//     }

//     let sqrt = 'outer: loop {
//         let n = gen_num();
//         for i in 1.. {
//             let square = i * i;
//             if square == n {
//                 break 'outer i;
//             }
//             if square > n {
//                 break;
//             }
//         }
//     };
//     println!("{}", sqrt);

//     loop {
//         if false {
//             return 1;
//         }
//     }
// }
// fn q_sort(array: &mut [u32]) {
//     if array.len() <= 1 {
//         return;
//     }

//     let pivot_index = array.len() / 2;
//     println!("array:{:?} pii{}", array, pivot_index);
//     let mut left_hold = 0;
//     let mut right_hold = array.len() - 1;
//     loop {
//         for i in left_hold..array.len() {
//             if array[i] > array[pivot_index] {
//                 left_hold = i;
//                 break;
//             }
//         }
//         for i in (0..right_hold + 1).rev() {
//             if array[i] < array[pivot_index] {
//                 right_hold = i;
//                 break;
//             }
//         }

//         if left_hold >= right_hold {
//             break;
//         } else {
//             println!("swap!l:{} r:{}", left_hold, right_hold);
//             array.swap(left_hold, right_hold);
//         }
//     }
//     q_sort(&mut array[..pivot_index]);
//     q_sort(&mut array[pivot_index + 1..]);
// }

// fn _quicksort() {
//     let mut array = [5, 6, 4, 3, 2];

//     q_sort(&mut array);
//     println!("sorted:{:?}", array);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sort() {
//         let mut array = [1, 2, 3, 4, 5];
//         q_sort(&mut array);
//         assert_eq!(array, [1, 2, 3, 4, 5]);

//         let mut array = [100, 99, 98, 97, 96];
//         q_sort(&mut array);
//         assert_eq!(array, [96, 97, 98, 99, 100]);
//     }
// }

// mod my_error {
//     use std::{fmt::Display, result};

//     use thiserror::{private::DisplayAsDisplay, Error};
//     pub type Result<T> = result::Result<T, PlaygroundError>;

//     #[derive(Debug, Error)]
//     pub enum PlaygroundError {
//         #[error("playground error: {0}")]
//         Custom(ErrorKind),
//         #[error("std lib error {0}")]
//         IoError(#[from] std::io::Error),
//     }

//     #[derive(Debug)]
//     pub enum ErrorKind {
//         UnknownError = 0x01,
//     }

//     impl ErrorKind {
//         pub fn as_str(&self) -> &'static str {
//             match self {
//                 UnknownError => "unknown error",
//             }
//         }
//     }

//     impl Display for ErrorKind {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             write!(f, "error_code {}", &self.as_str())
//         }
//     }
// }

// fn error_test(num: u32) -> my_error::Result<()> {
//     if num % 3 == 0 {
//         return Err(PlaygroundError::Custom(ErrorKind::UnknownError));
//     }

//     let path = Path::new("hello.txt");
//     File::open(&path)?;

//     Ok(())
// }

// fn _error_test_ext() {
//     if let Err(error) = error_test(99) {
//         println!("playground error {}", error.to_string());
//     }
// }

// mod util;
// fn _mod() {
//     util::print::print_hello();
// }

// fn _cell_sample() {
//     struct Sample {
//         counter: usize,
//         cell_counter: Cell<usize>,
//         log: RefCell<File>,
//     }

//     impl Sample {
//         fn new() -> Sample {
//             // let file = File::open("tmp").unwrap();
//             let file = OpenOptions::new()
//                 .create(true)
//                 .write(true)
//                 .open("tmp")
//                 .unwrap();
//             Sample {
//                 counter: 0,
//                 cell_counter: Cell::new(0),
//                 log: RefCell::new(file),
//             }
//         }

//         fn mut_execute(&mut self) {
//             self.counter = self.counter + 1;
//             println!("counter:{}", self.counter);
//         }

//         fn cell_execute(&self) {
//             self.cell_counter.set(self.cell_counter.get() + 1);
//             println!("cell_counter:{}", self.cell_counter.get());
//         }

//         fn write_log(&self) {
//             // self.log.write("abc".as_bytes());
//             let mut file = self.log.borrow_mut();
//             let mut temp = "abc".as_bytes();
//             file.write_all(temp).unwrap();
//             file.flush().unwrap();
//         }
//     }

//     let mut mut_sample = Sample::new();
//     mut_sample.mut_execute();

//     // cell merit is object is not mut, but can update internal member , caller handle objects as immutable object
//     let sample = Sample::new();
//     sample.cell_execute();
//     sample.cell_execute();
//     sample.write_log();
// }

// mod enum_sample;

// fn _enums() {
//     enum_sample::enum_sample();
// }

// mod traits;

// fn _traits() {
//     let mut localfile = File::create("hello.txt").unwrap();
//     traits::say_hello(&mut localfile);
//     traits::say_hello_g(&mut localfile);

//     let i_str = String::from("hello world");
//     traits::dump(i_str.chars());
// }

// mod overload;
// fn _overload() {
//     overload::overload_sample();
// }

use log::info;
// mod closure_sample;
// mod collection_sample;
// mod compress;
// mod iterator_sample;
use playground::package::compress;

fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
fn main() {
    init_logger();
    // exp_aup();

    // _quicksort();
    // _error_test_ext();
    // _mod();
    // _cell_sample();
    // _enums();
    // _traits();
    // _overload();
    {
        let start = Instant::now();
        // compress::compress_sample();

        compress::parallel_compress();
        let end = start.elapsed();
        info!("{}.{} sec", end.as_secs(), end.as_millis());
    }
    // closure_sample::sample();
    // iterator_sample::sample();
    // collection_sample::sample();
    // looptest();
    // movetest()
    // thread();
    // _reference();
}

// TODO:
// read file and then lz4 with fixed block
