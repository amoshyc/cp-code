#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }

    let mut ans = m;
    for mask in 0_usize..(1 << n) {
        if mask.count_ones() as usize == n {
            continue;
        }

        let mut cnt = 0;
        for (i, &(u, v)) in es.iter().enumerate() {
            if (mask >> u) & 1 == (mask >> v) & 1 {
                cnt += 1;
            }
        }
        ans = ans.min(cnt);
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
