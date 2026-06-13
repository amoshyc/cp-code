#![allow(unused)]

use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
        q: usize,
        asks: [(usize, usize); q],
    }

    let mut smaller = BTreeSet::new();
    let mut larger = BTreeSet::new();
    let mut ans = vec![];
    let mut cnt = 0;

    smaller.insert((x, 2 * q));
    cnt += 1;

    for i in 0..q {
        for j in [asks[i].0, asks[i].1] {
            smaller.insert((j, cnt));
            cnt += 1;

            // min(larger) should >= max(smaller)
            while !larger.is_empty() && !smaller.is_empty() {
                let (z1, id1) = *larger.first().unwrap();
                let (z2, id2) = *smaller.last().unwrap();
                if z1 < z2 {
                    larger.remove(&(z1, id1));
                    smaller.remove(&(z2, id2));
                    larger.insert((z2, id2));
                    smaller.insert((z1, id1));
                } else {
                    break;
                }
            }

            // Make smaller.len() = cnt / 2
            while smaller.len() < cnt / 2 && !larger.is_empty() {
                let z = larger.pop_first().unwrap();
                smaller.insert(z);
            }
    
            // make larger.len() = cnt / 2 + 1
            while larger.len() < cnt / 2 + 1 && !smaller.is_empty() {
                let z = smaller.pop_last().unwrap();
                larger.insert(z);
            }
        }

        ans.push(larger.first().unwrap().0);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
