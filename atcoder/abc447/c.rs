#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let split = |s: Vec<char>| -> VecDeque<(char, usize)> {
        let mut tokens = VecDeque::new();
        let mut i = 0;
        while i < s.len() {
            if s[i] == 'A' {
                let mut j = i;
                while j < s.len() && s[j] == 'A' {
                    j += 1;
                }
                tokens.push_back(('A', j - i));
                i = j;
            } else {
                tokens.push_back((s[i], 1));
                i += 1;
            }
        }
        tokens
    };

    // AAAWAZAAAABAAAU => A4, W1, A1, Z1, A4, B1, A3, U1
    // AWAAZABAAAAAUA => A1, W1, A2, Z1, A1, B1, A5, U1, A1
    let mut s_tokens = split(s.clone());
    let mut t_tokens = split(t.clone());

    let mut ans = 0;
    while let Some((&(a, x), &(b, y))) = s_tokens.front().zip(t_tokens.front()) {
        if a == 'A' && b == 'A' {
            ans += x.abs_diff(y);
            s_tokens.pop_front();
            t_tokens.pop_front();
        } else if a == 'A' {
            ans += x;
            s_tokens.pop_front();
        } else if b == 'A' {
            ans += y;
            t_tokens.pop_front();
        } else {
            if a != b {
                println!("-1");
                return;
            } else {
                s_tokens.pop_front();
                t_tokens.pop_front();
            }
        }
    }
    while let Some((a, x)) = s_tokens.pop_front() {
        if a == 'A' {
            ans += x;
        } else {
            println!("-1");
            return;
        }
    }
    while let Some((b, y)) = t_tokens.pop_front() {
        if b == 'A' {
            ans += y;
        } else {
            println!("-1");
            return;
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
