#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars
    }

    // l <= j - i <= r
    // j >= l + i
    // j <= r + i

    // j >= i

    let mut pos = vec![vec![]; 26];
    for i in 0..n {
        pos[s[i] as usize - 'a' as usize].push(i);
    }

    let mut ans = 0 as i64;
    for i in 0..n {
        let c = s[i] as usize - 'a' as usize;
        let p = pos[c].partition_point(|&x| x < l + i);
        let q = pos[c].partition_point(|&x| x <= r + i);
        ans += q as i64 - p as i64;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
