#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let map = "22233344455566677778889999".chars().collect::<Vec<char>>();
    let mut ans = vec![];
    for i in 0..n {
        let x = s[i][0] as usize - 'a' as usize;
        ans.push(map[x]);
    }

    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
