#![allow(unused)]

use std::f64::consts::PI;

fn main() {
    let n = read::<usize>();
    let inp = readv::<f64>();
    let (x0, y0) = (inp[0], inp[1]);
    let inp = readv::<f64>();
    let (xo, yo) = (inp[0], inp[1]);

    let cx = (x0 + xo) / 2.0;
    let cy = (y0 + yo) / 2.0;
    let dx = x0 - cx;
    let dy = y0 - cy;
    let angle = 2.0 * PI / (n as f64);
    let cos = angle.cos();
    let sin = angle.sin();
    let x1 = cx + cos * dx - sin * dy;
    let y1 = cy + sin * dx + cos * dy;
    println!("{:.6} {:.6}", x1, y1);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
