#![allow(warnings)]
use std::io;
use std::str::FromStr;

enum VecFormate {
    Inline,
    NotInline,
}


fn take_vec<T>(n: u32, vec_formate: VecFormate) -> Vec<T>
where
    T: FromStr /*+ std::fmt::Debug*/,
    T::Err: std::fmt::Debug, // Ensure the type implements FromStr and provides a debug error
{
    let mut v: Vec<T> = Vec::new();
    match vec_formate {
        VecFormate::Inline => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            v = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<T>().unwrap()) // Parse input into generic type
                .collect();
        }
        VecFormate::NotInline => {
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
    let size = 3;
    let v: Vec<u32> = take_vec(size, VecFormate::Inline);
    print!("{:?}", v);
}
