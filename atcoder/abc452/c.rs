#![allow(unused)]

use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(usize, Usize1); n],
        m: usize,
        arr_s: [Chars; m],
    }

    let ord = |c| c as usize - 'a' as usize;

    // can[c][p][l] = Are there a char `c` at position `p` where the string length is `l`
    let mut can = vec![vec![vec![false; 11]; 10]; 26];
    for s in &arr_s {
        for (p, &c) in s.iter().enumerate() {
            can[ord(c)][p][s.len()] = true;
        }
    }

    let mut ans = vec!["No"; m];
    for i in 0..m {
        if arr_s[i].len() == n {
            let mut ok = true;
            for (p, &c) in arr_s[i].iter().enumerate() {
                let (a, b) = ab[p];
                ok &= can[ord(c)][b][a];
            }
            if ok {
                ans[i] = "Yes";
            }
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
