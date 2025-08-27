#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize, m: usize,
        arr: [Chars; n],
    }

    let mut votes = vec![0; n];
    for r in 0..m {
        let x = (0..n).filter(|&i| arr[i][r] == '0').count();
        let y = n - x;

        if x == 0 && y == 0 {
            for i in 0..n {
                votes[i] += 1;
            }
        } else if x < y {
            for i in 0..n {
                if arr[i][r] == '0' {
                    votes[i] += 1;
                }
            }
        } else {
            for i in 0..n {
                if arr[i][r] == '1' {
                    votes[i] += 1;
                }
            }
        }
    }

    let max = *votes.iter().max().unwrap();
    let ans = (1..=n).filter(|&i| votes[i - 1] == max).collect::<Vec<_>>();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
