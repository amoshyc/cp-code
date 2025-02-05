#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<i64>();
    arr.sort();

    let f = |p: usize| -> usize {
        let smaller = arr[..p].to_vec();
        let larger = arr[p..].to_vec();

        let mut cnt = 0;
        let mut j = 0;
        for i in 0..smaller.len() {
            while j < larger.len() && larger[j] < 2 * smaller[i] {
                j += 1;
            }

            if j < larger.len() {
                cnt += 1;
            }

            j += 1;
        }

        cnt
    };

    let mut lb = 0;
    let mut ub = n - 1;
    while ub - lb > 8 {
        let l = (lb + lb + ub) / 3;
        let r = (lb + ub + ub) / 3;
        if f(l) >= f(r) {
            ub = r;
        } else {
            lb = l;
        }
    }

    let mut ans = 0;
    for i in lb..=ub {
        ans = ans.max(f(i));
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
