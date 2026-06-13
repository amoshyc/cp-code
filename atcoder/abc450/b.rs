#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cost = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        input! {
            inp: [i64; n - 1 - i],
        }
        cost[i][i + 1..].copy_from_slice(&inp);
    }

    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if cost[a][b] + cost[b][c] < cost[a][c] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
