#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
        asks: [(usize, usize); q],
    }

    let mut ans = vec![];
    for (ask, x) in asks {
        if ask == 1 {
            ans.push(w * x);
            h -= x;
        } else {
            ans.push(h * x);
            w -= x;
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
