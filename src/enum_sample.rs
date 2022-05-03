use std::{collections::HashMap, mem::size_of, str::FromStr};

#[derive(Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    // Object(Box<HashMap<String, Json>>),
}

pub fn enum_sample() {
    let null = Json::Null;
    println!("size of {}", size_of::<Json>());
    let json_true = Json::Boolean(true);
    let json_str = Json::String(String::from_str("test").unwrap());
    let array = vec![null, json_true, json_str];
    let array = Json::Array(array);
    println!("array:{:?}", array);
    match &array {
        Json::Array(ref val) => {
            for json in val {
                json.print();
            }
        }
        _ => {}
    }
    array.print();
}

impl Json {
    pub fn print(&self) {
        match self {
            Json::Null => {
                println!("json is null");
            }
            Json::Boolean(val) if *val == true => {
                println!("json is bool true {}", val);
            }
            other => {
                println!("not {:?}", other);
            }
        }
    }
}
