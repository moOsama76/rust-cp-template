use std::io;
use std::str::FromStr;

fn take_number<T>() -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn take_array<T>(n: u32) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug, // Ensure the type implements FromStr and provides a debug error
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<T> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap()) // Parse input into generic type
        .collect();
    
    if v.len() as u32 > n {
        panic!("You added additional {:?} items", v.len() as u32 - n);
    } else if n > v.len() as u32 {
        panic!("You missed remaining {:?} items", n - v.len() as u32);
    }

    v
}

fn main() {
    let n: u32 = take_number();
    let v: Vec<u32> = take_array(n);
    let m: u32 = take_number();
    print!("{:?} \n{:?}\n{:?}", n, v, m);
}
