#![allow(warnings)]
use std::io;
use std::str::FromStr;
fn take_item<T>() -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let n: u32 = take_item();
    print!("{:?}", n);
}
