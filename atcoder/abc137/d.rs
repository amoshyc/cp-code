#![allow(unused)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        arr: [(i64, i64); n],
    }

    // job i can be done in days 1..=(m - a[i])

    //                Day
    //             0 1 2 3 4
    // job A (3):  o
    // job B (1):  o
    // job C (2):  o o o

    let mut boxes = vec![];
    for i in 0..m {
        boxes.push(i as i64);
    }

    let mut items = vec![];
    for (i, &(a, b)) in arr.iter().enumerate() {
        items.push((m as i64 - a, b));
    }

    println!("{}", greedy_packing_10(items, boxes));
}

//             Sorted Boxes
//               A B C D
// item 0 (v0):  1
// item 1 (v1):  1 1 1
// item 2 (v2):  1 1 1 1
// item 3 (v3):  1 1
// Each row (item) is a 1...0 distribution.
fn greedy_packing_10(mut items: Vec<(i64, i64)>, boxes: Vec<i64>) -> i64 {
    let mut boxes = BTreeSet::from_iter((0..boxes.len()).map(|i| (boxes[i], i)));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
