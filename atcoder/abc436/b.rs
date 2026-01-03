#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    }

    let mut arr = vec![vec![0; n]; n];
    let mut r = 0;
    let mut c = (n - 1) / 2;
    arr[r][c] = 1;
    for i in (2..=(n * n)) {
        let mut nr = (r + n - 1) % n;
        let mut nc = (c + 1) % n;
        if arr[nr][nc] != 0 {
            nr = (r + 1) % n;
            nc = c;
        }
        arr[nr][nc] = i;
        (r, c) = (nr, nc);
    }

    for r in 0..n {
        println!("{}", join(&arr[r], " "));
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
