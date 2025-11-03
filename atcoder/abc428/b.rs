#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut ans = vec![];
    let mut max = 0;
    for i in (0..).take_while(|i| i + k <= n) {
        let cnt = (0..n)
            .take_while(|j| j + k <= n)
            .filter(|&j| s[j..j + k] == s[i..i + k])
            .count();
        ans.push((cnt, s[i..i + k].to_vec()));
        max = max.max(cnt);
    }

    let mut ans = ans
        .iter()
        .filter(|&(c, p)| *c == max)
        .map(|(c, p)| join(&p, ""))
        .collect::<Vec<_>>();
    ans.sort();
    ans.dedup();
    println!("{max}");
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
