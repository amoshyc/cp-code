#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut arr: [usize; n],
        ask: [(usize, usize); q],
    }

    arr.insert(0, 0);
    let n = n + 1;

    let mut nxt = vec![vec![0; n]; 30];
    let mut sum = vec![vec![0; n]; 30];

    nxt[0] = arr;
    for u in 0..n {
        sum[0][u] = u as i64;
    }

    for i in 1..30 {
        for u in 0..n {
            let v = nxt[i - 1][u];
            nxt[i][u] = nxt[i - 1][v];
            sum[i][u] = sum[i - 1][u] + sum[i - 1][v];
        }
    }

    let mut ans = vec![];
    for &(t, b) in &ask {
        let mut v = 0;
        let mut x = b;
        for i in 0..30 {
            if (t >> i) & 1 == 1 {
                v += sum[i][x];
                x = nxt[i][x];
            }
        }
        ans.push(v);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
