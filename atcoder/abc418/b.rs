#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut ans: f64 = 0.0;
    for l in 0..n {
        for r in (l + 2)..n {
            if s[l] == 't' && s[r] == 't' {
                let cnt = (l..=r).filter(|&i| s[i] == 't').count();
                ans = ans.max((cnt as f64 - 2.0) / ((r - l + 1 - 2) as f64));
            }
        }
    }

    println!("{:.10}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
