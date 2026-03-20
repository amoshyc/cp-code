#![allow(unused)]

use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut pos_b = (0..n).filter(|&i| s[i] == 'B').collect::<BTreeSet<_>>();
    let mut pos_c = (0..n).filter(|&i| s[i] == 'C').collect::<BTreeSet<_>>();

    let mut ans = 0;
    for i in 0..n {
        if s[i] == 'A' {
            if let Some(&j) = pos_b.range(i..).next() {
                if let Some(&k) = pos_c.range(j..).next() {
                    ans += 1;
                    pos_b.remove(&j);
                    pos_c.remove(&k);
                }
            }
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
