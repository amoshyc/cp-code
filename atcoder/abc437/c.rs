#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            mut wp: [(i64, i64); n],
        }

        wp.sort_by_key(|&(w, p)| p + w);

        let mut total_w = 0;
        let mut total_p = wp.iter().map(|&(w, p)| p).sum::<i64>();

        let mut max = 0;
        for i in 0..n {
            total_w += wp[i].0;
            total_p -= wp[i].1;
            if total_w <= total_p {
                max = i + 1;
            }
        }

        ans.push(max);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
