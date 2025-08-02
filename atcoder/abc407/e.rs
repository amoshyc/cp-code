#![allow(unused)]

use std::collections::BinaryHeap;

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![0; tc];
    for i in 0..tc {
        let n = read::<usize>();
        let mut arr = vec![0; 2 * n];
        for i in 0..(2 * n) {
            arr[i] = read::<i64>();
        }

        let mut heap = BinaryHeap::new();
        let mut seq = vec![')'; 2 * n];

        seq[0] = '(';
        let mut cnt = 1;

        for i in 1..(2 * n - 1) {
            heap.push((arr[i], i));
            if cnt < i / 2 + 1 {
                let (_, j) = heap.pop().unwrap();
                seq[j] = '(';
                cnt += 1;
            }
        }

        ans[i] = (0..2 * n)
            .filter(|&i| seq[i] == '(')
            .map(|i| arr[i])
            .sum::<i64>();
    }
    println!("{}", join(&ans, "\n"));
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
