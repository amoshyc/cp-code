#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
    }

    let fes = [(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];
    if fes.contains(&(m, d)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
