use std::vec;

struct Data {
    x: usize,
}

pub fn sample() {
    let mut v = vec![1, 2, 3, 4, 5];
    let a = 32;
    println!("{:p} {:p} {:p}", &v[0], &v[1], &a);
    v.push(a);
    println!("{:p} {:p} {:p}", &v[0], &v[1], &v[5]);
    assert_eq!(v[5], a);

    let test = &v[1..3];
    println!("{:?}", test.binary_search(&10));

    let mut v = vec![Data { x: 1 }, Data { x: 100 }];
    let d = Data { x: 200 };
    println!("{:p} {:p}", &v[0], &d);
    v.push(d);
    println!("{:p} {:p}", &v[0], &v[2]);
}
