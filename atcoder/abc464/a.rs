#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        s: Chars,
    }

    let cnt_e = s.iter().filter(|&&c| c == 'E').count();
    let cnt_w = s.len() - cnt_e;
    if cnt_e > cnt_w {
        println!("East");
    } else {
        println!("West");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
