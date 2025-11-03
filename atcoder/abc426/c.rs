#![allow(unused)]

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(usize, usize); q],
    }

    let mut cnt = BTreeMap::new();
    for i in 1..=n {
        cnt.insert(i, 1);
    }

    let mut ans = vec![];
    for (x, y) in xy {
        let mut pairs = vec![];
        for (k, v) in cnt.iter() {
            if *k > x {
                break;
            } else {
                pairs.push((*k, *v));
            }
        }
        let mut val = 0;
        for (k, v) in pairs {
            val += v;
            cnt.remove(&k);
        }

        ans.push(val);
        *cnt.entry(y).or_insert(0) += val;
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
