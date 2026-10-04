#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        pos: [[i64; m]; n],
    }

    let mut ans = 0;
    for d in 0..m {
        let xs = (0..n).map(|i| pos[i][d]).collect();
        ans += solve_1d(xs);
    }
    println!("{ans}");
}

fn solve_1d(mut xs: Vec<i64>) -> i64 {
    xs.sort();
    let mut ans = 0;
    let mut pref = 0;
    for i in 0..xs.len() {
        ans += i as i64 * xs[i] - pref;
        pref += xs[i];
    }
    ans
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
