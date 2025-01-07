#![allow(warnings)]
use std::io;
use std::str::FromStr;

enum ArrayFormate {
    Inline,
    NotInline,
}

fn take_item<T>() -> T
where
    T: FromStr /*+ std::fmt::Debug*/,
    T::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn take_array<T>(n: u32, array_formate: ArrayFormate) -> Vec<T>
where
    T: FromStr /*+ std::fmt::Debug*/,
    T::Err: std::fmt::Debug, // Ensure the type implements FromStr and provides a debug error
{
    let mut v: Vec<T> = Vec::new();
    match array_formate {
        ArrayFormate::Inline => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            v = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<T>().unwrap()) // Parse input into generic type
                .collect();
        }
        ArrayFormate::NotInline => {
            // We don't need to read an extra line before the loop
            for _ in 0..n {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                v.push(s.trim().parse::<T>().unwrap()); // Parse input into generic type and push to vector
            }
        }
    }
    if v.len() as u32 > n {
        panic!("You added additional {:?} items", v.len() as u32 - n);
    } else if n > v.len() as u32 {
        panic!("You missed remaining {:?} items", n - v.len() as u32);
    }
    v
}

fn main() {
    let n: u32 = take_item();
    let v: Vec<String> = take_array(n, ArrayFormate::Inline);
    let m: u32 = take_item();
    print!("{:?}\n{:?}\n{:?}", n, v, m);
}
