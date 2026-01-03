#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        arr: [[usize; w]; h],
        bbb: [usize; n]
    }

    let ans = (0..h)
        .map(|r| (0..w).filter(|&c| bbb.contains(&arr[r][c])).count())
        .max()
        .unwrap();

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
