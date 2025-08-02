#![allow(unused)]

use std::collections::BinaryHeap;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    // Greedily select the maximum one

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((arr[i], i, 0));
    }

    for _ in 0..m {
        if let Some((v, i, k)) = heap.pop() {
            let new_val = arr[i] / (1 << (k + 1));
            if new_val > 0 {
                heap.push((new_val, i, k + 1));
            }
        }
    }

    let mut ans = 0;
    for &(_, i, k) in &heap {
        ans += arr[i] / (1 << k);
    }
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
