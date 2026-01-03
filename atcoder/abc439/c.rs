#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cnt = vec![0; n + 1];
    for x in (1..).take_while(|&x| x * x <= n) {
        for y in ((x + 1)..).take_while(|&y| x * x + y * y <= n) {
            cnt[x * x + y * y] += 1;
        }
    }

    let res = (1..=n).filter(|&x| cnt[x] == 1).collect::<Vec<_>>();
    println!("{}", res.len());
    println!("{}", join(&res, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
