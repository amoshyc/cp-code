#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut arr_a: [i64; n],
        mut arr_b: [i64; n],
        asks: [(char, Usize1, i64); q],
    }

    let mut min = (0..n).map(|i| arr_a[i].min(arr_b[i])).collect::<Vec<_>>();
    let mut sum = min.iter().sum::<i64>();
    let mut ans = vec![];
    for &(c, x, v) in &asks {
        if c == 'A' {
            arr_a[x] = v;
        } else {
            arr_b[x] = v;
        }
        let new_min = arr_a[x].min(arr_b[x]);
        if new_min != min[x] {
            sum -= min[x];
            sum += new_min;
            min[x] = new_min;
        }
        ans.push(sum);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
