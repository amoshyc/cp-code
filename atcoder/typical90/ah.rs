#![allow(unused)]

// [Problem]
// Given a sequence a=(a1,a2,…,aN) of length N and an integer K, you need to find the length of the longest contiguous subsequence that contains at most K distinct values.

// [Solution]
// Two pointers with HashMap to find
//     ans[i] = length of the longest substring starting from i with <= K unique elements.

use std::collections::HashMap;

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut ans = 0;
    let mut cnt = HashMap::new();

    let mut i = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && cnt.len() + if cnt.contains_key(&arr[j]) { 0 } else { 1 } <= k {
            *cnt.entry(arr[j]).or_insert(0) += 1;
            j += 1;
        }
        
        ans = ans.max(j - i);

        *cnt.entry(arr[i]).or_insert(0) -= 1;
        if cnt[&arr[i]] == 0 {
            cnt.remove_entry(&arr[i]);
        }
    }

    println!("{}", ans);
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
    read::<String>().chars().collect()
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
