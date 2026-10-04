#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        tc: usize,
        asks: [(i64, i64, i64); tc]
    }

    let mut res = vec![];
    for &(x, y, k) in &asks {
        let mut xs = HashMap::new();

        xs.insert(x, 0);

        if x != 0 {
            let mut val = x;
            for i in 1.. {
                val /= k;
                xs.insert(val, i);
                if val == 0 {
                    break;
                }
            }
        }

        let mut val = y;
        for i in 0.. {
            if let Some(s) = xs.get(&val) {
                res.push(s + i as i64);
                break;
            }
            val /= k;
        }
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
