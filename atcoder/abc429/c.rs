#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut groups = vec![0; n + 1];
    for i in 0..n {
        groups[arr[i]] += 1;
    }

    let mut ans = 0 as i64;
    for cnt in groups {
        ans += cnt * (cnt - 1) / 2 * (n as i64 - cnt);
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
