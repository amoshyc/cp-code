#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut ans = 0 as i64;

    for i in 0..n {
        if s[i] == 'C' {
            let l = i;
            let r = n - i - 1;
            ans += l.min(r) as i64;
            ans += 1;
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
