#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [i64; n],
    }

    let mut ans = 0;
    for l in 0..n {
        for r in l..n {
            let sum = arr[l..=r].iter().sum::<i64>();
            if (l..=r).all(|i| sum % arr[i] != 0) {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
