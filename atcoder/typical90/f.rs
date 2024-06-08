#![allow(unused)]

// [Problem]
// You are given a string S of length N consisting of lowercase English letters.
// Please output the lexicographically smallest subsequence of S with length K.

// [Solution]
// In lexicographic order, azzzzz < baaaaa, this suggest that we may use greedy algorithm:
// If the answer is T, find the smallest T[0], and then T[1], and then T[2], ...
// What's the smallest possible T[0]?
// It is the smallest character of S[0..=(N - 1 - (K - 1))] because we still need to pick K - 1 character except this one.
// Formally, for T[i], the candidates are S[(prev + 1)..=(N - 1 - (K - i - 1))] where prev is the chosen index of the (i - 1)-th char.
// 
// For implementation, we can build a table that stores the positions of each char from 'a' to 'z'.
// Then we can enumerate 'a' to 'z' and select the first one that has a position in the valid range.
// Time complexity is O(N * 26 * lg(N)).
// It can be improved to O(N * 26). Please check official editorial for detail.

use std::collections::{BTreeSet, HashMap};

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let s = reads();

    let mut pos = HashMap::new();
    for i in 0..n {
        pos.entry(s[i]).or_insert(BTreeSet::new()).insert(i);
    }

    let mut ans = vec![];
    let mut prev = 0;
    for i in 0..k {
        let range_l = if i == 0 { 0 } else { prev + 1 };
        let range_r = n - k + i;
        for c in 'a'..='z' {
            if let Some(c_pos) = pos.get(&c) {
                if let Some(p) = c_pos.range(range_l..=range_r).next() {
                    ans.push(c);
                    prev = *p;
                    break;
                }
            }
        }
    }

    println!("{}", join(&ans, ""));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<_>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
