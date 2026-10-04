#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // ai + b
    // a (c * i + d) + b = ac * i + (ad + b)

    let mut a = 0;
    let mut b = 0;
    let mut ans = vec![0; n];
    for k in (0..n).rev() {
        if s[k] == 'o' {
            let (c, d) = (-1, k as i64);
            if (a, b) == (0, 0) {
                (a, b) = (c, d);
            } else {
                let new_a = a * c;
                let new_b = a * d + b;
                (a, b) = (new_a, new_b);
            }
        }

        let p = if (a, b) == (0, 0) {
            k as i64
        } else {
            k as i64 * a + b
        };

        ans[p as usize] = k + 1;
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
