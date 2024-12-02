#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let s = readv::<i32>();

    // even
    let mut even = vec![];
    for i in (1..n).step_by(2) {
        if s[i] == s[i - 1] {
            even.push(s[i]);
        } else {
            even.push(-1);
        }
    }

    // odd
    let mut odd = vec![];
    for i in (2..n).step_by(2) {
        if s[i] == s[i - 1] {
            odd.push(s[i]);
        } else {
            odd.push(-1);
        }
    }

    println!("{}", solve(&even).max(solve(&odd)) * 2);
}

fn solve(arr: &Vec<i32>) -> usize {
    let n = arr.len();
    let mut set = HashSet::new();
    let mut ans = 0;
    let mut j = 0;
    for i in 0..n {
        if arr[i] != -1 {
            set.insert(arr[i]);
            j = j.max(i + 1);
            while j < n && arr[j] != -1 && !set.contains(&arr[j]) {
                set.insert(arr[j]);
                j += 1;
            }
            ans = ans.max(j - i);
            set.remove(&arr[i]);
        }
    }
    ans
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
