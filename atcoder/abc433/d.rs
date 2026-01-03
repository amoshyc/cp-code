#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        arr: [i64; n],
    }

    // x * 10^d + y == 0 (mod m)
    // x * 10^d == -y (mod m)

    let mut pow10 = vec![1; 11];
    for i in 1..11 {
        pow10[i] = pow10[i - 1] * 10 % m;
    }

    // for each d, maintain the number of x * 10^d % m
    let mut cnt = vec![HashMap::new(); 11];
    for i in 0..n {
        for d in 0..11 {
            let val = (arr[i] % m * pow10[d]) % m;
            *cnt[d].entry(val).or_insert(0) += 1 as i64;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let d = digits(arr[i], 10).len();
        let rhs = (-arr[i]).rem_euclid(m);
        ans += cnt[d].get(&rhs).unwrap_or(&0);
    }

    println!("{ans}");
}

fn digits(mut x: i64, a: i64) -> Vec<i64> {
    if x == 0 {
        return vec![0];
    }
    let mut res = vec![];
    while x != 0 {
        res.push(x % a);
        x /= a;
    }
    res.reverse();
    res
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
