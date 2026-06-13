#![allow(unused)]

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut i = 0;
    let mut j = 1;
    let mut ans = 0;
    for i in 0..n {
        j = j.max(i + 1);
        while j < n && s[j] != s[j - 1] {
            j += 1;
        }
        ans += (j - i) as i64;
        ans %= 998244353;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
