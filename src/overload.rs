use std::ops::Add;

pub fn overload_sample() {
    assert_eq!(4.5f32.add(2.3), 6.8);
    println!("{:b}", !0xABC_DEE);
}
