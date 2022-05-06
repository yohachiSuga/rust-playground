use rand::random;
use std::{
    collections::BTreeMap,
    iter::{from_fn, once, repeat},
};

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

pub fn sample() {
    assert_eq!(1, triangle(1));
    assert_eq!(3, triangle(2));

    let rands: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();
    println!("{:?}", rands[0]);

    let text = "ab \n harry potter \n test";
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| (*s).len() >= 3)
        .collect();
    println!("mapped v:{:?}", v);

    let mut parks = BTreeMap::new();
    let myoden_park = vec!["daiichi", "daini"];
    parks.insert("myoden", myoden_park);
    parks.insert("gyotoku", vec!["ekimaehiroba", "daini"]);
    let all: Vec<_> = parks.values().flatten().collect();
    println!("{:?}", all);
    // println!("{:?}",myoden_park);

    let body = "TO AAA\r\n\
    FROM MARIA \r\n\
    \r\n\
    Hi, AAA\r\n\
    How are you?\r\n";
    for m in body.lines().take_while(|l| !l.is_empty()) {
        println!("{}", m);
    }
    for m in body.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", m);
    }

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let v: Vec<_> = (1..10).zip(fizzes_buzzes).collect();
    println!("{:?}", v);

    let num = [1, 23, 4, 5];
    let sum: usize = num.iter().sum();
    let prod: usize = num.iter().product();
    println!(
        "sum:{}, prod:{}, min:{:?},max:{}",
        sum,
        prod,
        num.iter().min().unwrap(),
        num.iter().max().unwrap()
    );

    println!("count:{}", num.iter().fold(0, |n, _| n + 1));
    println!("sum:{}", num.iter().fold(0, |n, i| n + i));
    println!("prod:{}", num.iter().fold(1, |n, i| n * i));

    struct SampleRange {
        start: i32,
        end: i32,
    };
    impl Iterator for SampleRange {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.start >= self.end {
                return None;
            }
            let result = Some(self.start);
            self.start += 1;
            result
        }
    };

    for i in (SampleRange { start: 0, end: 5 }) {
        println!("{}", i);
    }
}
