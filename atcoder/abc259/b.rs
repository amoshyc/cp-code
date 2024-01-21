#![allow(unused)]

use std::f64::consts::PI;

fn main() {
    let inp = readv::<f64>();
    let (a, b, d) = (inp[0], inp[1], inp[2]);

    let d = d / 180.0 * PI;
    let s = d.sin();
    let c = d.cos();
    let x = c * a - s * b;
    let y = s * a + c * b;
    println!("{:.8} {:.8}", x, y);
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
