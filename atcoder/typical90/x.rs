#![allow(unused)]

// [Problem]
// You are given a sequence of positive integers A=(A1, A2, ..., AN) and B=(B1, B2, ..., BN).
// Determine whether you can make A match B by performing the following operation exactly K times.
// Operation:
// Choose an i that satisfies 1≤i≤N and replace A[i] with A[i]-1 or A[i]+1.

// [Solution]
// Minimum number of operation needed is sum_i abs(B[i] - A[i]).
// if it is <= K and has same parity as K, we can apply +1 and -1 on a single elements.

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1] as i64);
    let arr_a = readv::<i64>();
    let arr_b = readv::<i64>();
    let m = (0..n).map(|i| (arr_b[i] - arr_a[i]).abs()).sum::<i64>();
    if m <= k && m % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
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
    read::<String>().chars().collect::<Vec<char>>()
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
