#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // 1 2 3 4 5 6 7 8 9
    // 2 9 9 7 9 2 4 5 8
    //       ^     ^ ^ ^

    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut cnt = HashMap::new();
    for &x in &arr {
        *cnt.entry(x).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut ans = 0;
    for i in 0..n {
        if cnt[&arr[i]] == 1 {
            if arr[i] > max {
                max = arr[i];
                ans = i + 1;
            }
        }
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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
