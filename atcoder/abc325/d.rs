#![allow(unused)]

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        arr: [(i64, i64); n],
    }

    let segs = arr.iter().map(|&(i, d)| (i, i + d)).collect();
    let ans = greedy_matching_segs(segs);
    println!("{ans}");
}

//            Boxes
//          0 1 2 3 4
// item 0:    o---o
// item 1:  o-----o
// item 2:      o-o
// item 3:      o---o
fn greedy_matching_segs(mut segs: Vec<(i64, i64)>) -> usize {
    segs.sort();
    let ls = BTreeSet::from_iter(segs.iter().map(|s| s.0));
    let mut rs = BTreeSet::new();
    let mut t = 0; // sweep line
    let mut i = 0; // segs index
    let mut ans = 0;
    loop {
        // Insert new valid segments to rs
        while i < segs.len() && segs[i].0 <= t {
            rs.insert((segs[i].1, i));
            i += 1;
        }
        // Remove invalid segments from rs
        while let Some(&(r, j)) = rs.first() {
            if r < t {
                rs.remove(&(r, j));
            } else {
                break;
            }
        }
        // Try to pick 1 segment
        if let Some(&(r, j)) = rs.first() {
            // Successfully picked
            rs.remove(&(r, j));
            ans += 1;
            t += 1;
        } else {
            // Jump to next possible time
            if let Some(&l) = ls.range(t + 1..).next() {
                t = l;
            } else {
                break;
            }
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
