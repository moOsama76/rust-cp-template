
#![allow(warnings)]
use std::io;
use std::str::FromStr;

enum ArrFormate {
    Inline,
    NotInline,
}

fn take_array<T, const MAX: usize>(n: usize, array_formate: ArrFormate) -> [T; MAX]
where
    T: FromStr + Default + Copy,
    T::Err: std::fmt::Debug,
{
    if n > MAX {
        panic!("Input size exceeds the maximum allowed array size of {}", MAX);
    }

    let mut a: [T; MAX] = [T::default(); MAX];
    match array_formate {
        ArrFormate::Inline => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let parsed: Vec<T> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<T>().unwrap())
                .collect();

            if parsed.len() != n {
                panic!(
                    "Expected {} items, but got {}",
                    n,
                    parsed.len()
                );
            }

            for (i, value) in parsed.into_iter().enumerate() {
                a[i] = value;
            }
        }
        ArrFormate::NotInline => {
            for i in 0..n {
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
                a[i] = s.trim().parse::<T>().unwrap();
            }
        }
    }
    a
}

fn prefix_sum<T, const MAX: usize>(mut a: [T; MAX], n: usize) -> [T; MAX]
where
    T: Copy + std::ops::AddAssign + Default,
{
    let mut sum = T::default();
    for i in 0..n {
        sum += a[i];
        a[i] = sum;
    }
    a
}

fn main() {
    const MAX: usize = 100;
    let size = 3;
    let mut a: [u64; MAX] = take_array(size, ArrFormate::Inline);
    print!("{:?}", a);

}
