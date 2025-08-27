#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut arr: [i64; n],
        ask: [i64; q],
    }

    arr.sort();
    let mut pref = vec![arr[0]];
    for i in 1..n {
        pref.push(pref[i - 1] + arr[i]);
    }

    let mut ans = vec![];
    for b in ask {
        let i = arr.partition_point(|&x| x < b);
        let left = if i >= 1 { pref[i - 1] } else { 0 };
        let right = (n - i) as i64 * (b - 1);
        let val = left + right + 1;

        if val > pref[n - 1] {
            ans.push(-1);
        } else {
            ans.push(val);
        }
    }
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
