use std::fmt::Debug;
use std::io::Write;

// dyn means dynamic dispatch (use vtable like C++)
pub fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello World\n")?;
    out.flush()
}

// not use dynamic dispatch, but it increases binary size like C++.
pub fn say_hello_g<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(b"Hello World\n")?;
    out.flush()
}

trait Vegetable {}

struct Salad {
    veggies: Vec<Box<dyn Vegetable>>,
}

pub fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (i, v) in iter.enumerate() {
        println!("{},{:?}", i, v);
    }
}
