#![allow(unused)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        arr_p: [i64; n],
        arr_l: [i64; m],
        arr_d: [i64; m],
    }

    //                  Price
    //                  1 3 4
    // discount 0 (2):      o
    // discount 1 (3):      o
    // discount 2 (1):    o o

    let mut items = vec![];
    for i in 0..m {
        items.push((arr_l[i], arr_d[i]));
    }
    let total_discount = greedy_packing_01(items, arr_p.clone());
    let ans = arr_p.iter().sum::<i64>() - total_discount;
    println!("{ans}");
}

//             Sorted Boxes
//               A B C D
// item 0 (v0):  1
// item 1 (v1):  1 1 1
// item 2 (v2):  1 1 1 1
// item 3 (v3):  1 1
// Each row (item) is a 1...0 distribution.
fn greedy_packing_10(mut items: Vec<(i64, i64)>, bounds: Vec<i64>) -> i64 {
    let mut boxes = std::collections::BTreeSet::new();
    for (i, &b) in bounds.iter().enumerate() {
        boxes.insert((b, i));
    }
    items.sort_by_key(|&(w, v)| (std::cmp::Reverse(v), w));
    let mut ans = 0;
    for &(w, v) in &items {
        if let Some(&(b, i)) = boxes.range(..=(w, !0)).last() {
            boxes.remove(&(b, i));
            ans += v;
        }
    }
    ans
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
