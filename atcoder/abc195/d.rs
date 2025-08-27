#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        items: [(i64, i64); n],
        boxes: [i64; m],
        asks: [(Usize1, Usize1); q],
    }

    let mut ans = vec![];
    for &(l, r) in &asks {
        let subset = (0..m)
            .filter(|&i| i < l || i > r)
            .map(|i| boxes[i])
            .collect();
        ans.push(greedy_packing_01(items.clone(), subset));
    }

    println!("{}", join(&ans, "\n"));
}

//             Sorted Boxes
//               A B C D
// item 0 (v0):      1 1
// item 1 (v1):    1 1 1
// item 2 (v2):        1
// item 3 (v3):  1 1 1 1
// Each row (item) is a 0...1 distribution.
fn greedy_packing_01(mut items: Vec<(i64, i64)>, boxes: Vec<i64>) -> i64 {
    let mut boxes = BTreeSet::from_iter((0..boxes.len()).map(|i| (boxes[i], i)));
    items.sort_by_key(|&(w, v)| (Reverse(v), w));
    let mut ans = 0;
    for &(w, v) in &items {
        if let Some(&(b, i)) = boxes.range((w, 0)..).next() {
            boxes.remove(&(b, i));
            ans += v;
        }
    }
    ans
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
