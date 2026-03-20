#![allow(unused)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        asks: [Chars; q],
    }

    let s = HashSet::<char>::from_iter(s.iter().cloned());
    let t = HashSet::<char>::from_iter(t.iter().cloned());

    let mut ans = vec![];
    for w in asks {
        let all_in_s = w.iter().all(|c| s.contains(c));
        let all_in_t = w.iter().all(|c| t.contains(c));

        if all_in_s && !all_in_t {
            ans.push("Takahashi");
        } else if all_in_t && !all_in_s {
            ans.push("Aoki");
        } else {
            ans.push("Unknown")
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
