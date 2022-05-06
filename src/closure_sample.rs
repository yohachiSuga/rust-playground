pub fn sample() {
    fn calc_specify_fn(num: usize, calc: fn(num: usize) -> usize) {
        println!("{}", calc(num));
    }
    fn pow(n: usize) -> usize {
        n * n
    };
    calc_specify_fn(2, pow);
    calc_specify_fn(2, |n| n * n);

    fn call_twice<F>(mut closure: F)
    where
        F: FnMut(),
    {
        closure();
        closure();
    }

    call_twice(|| println!("hello twice!"));

    // let hello = "hello".to_string();
    // let f = || drop(hello);
    // call_twice(f);

    let mut i = 0;
    let f = || {
        i += 1;
        println!("{}", i);
    };
    call_twice(f);
}
