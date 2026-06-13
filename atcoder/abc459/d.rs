#![allow(unused)]

use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        q: usize,
        s: [Chars; q],
    }

    let mut ans = vec![];
    for s in s {
        let n = s.len();
        let mut cnt = vec![0 as usize; 26];
        for &c in &s {
            cnt[c as usize - 'a' as usize] += 1;
        }

        let mut indices = (0..26).collect::<Vec<_>>();
        indices.sort_by_key(|&i| cnt[i]);

        let mut chars = vec![];
        for &i in indices.iter().rev() {
            let c = (i as u8 + 'a' as u8) as char;
            chars.extend(vec![c; cnt[i]]);
        }

        // interleave
        let res = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    chars[i / 2]
                } else {
                    chars[(n + 1) / 2 + i / 2]
                }
            })
            .collect::<Vec<char>>();

        // check
        let ok = res.windows(2).all(|w| w[0] != w[1]);
        if !ok {
            ans.push("No".to_string());
        } else {
            ans.push("Yes".to_string());
            ans.push(join(&res, ""));
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
