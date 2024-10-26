#![allow(unused)]

use std::collections::BinaryHeap;

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];

    for _ in 0..tc {
        let inp = readv::<usize>();
        let (n, k) = (inp[0], inp[1]);
        let mut arr_a = readv::<i64>();
        let mut arr_b = readv::<i64>();

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|i| arr_a[*i]);

        let arr_a = mapv(&indices, |i| arr_a[*i]);
        let arr_b = mapv(&indices, |i| arr_b[*i]);

        let mut que = BinaryHeap::new();
        let mut sum = 0;
        let mut min = 10i64.pow(18);

        for i in 0..n {
            if i >= 1 {
                que.push(arr_b[i - 1]);
                sum += arr_b[i - 1];
                if que.len() > k - 1 {
                    sum -= que.pop().unwrap();
                }
            }
            if que.len() >= k - 1 {
                min = min.min(arr_a[i] * (sum + arr_b[i]));
            }
        }
        ans.push(min);
    }

    println!("{}", join(&ans, "\n"));
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
