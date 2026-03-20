#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        items: [(usize, usize); n],
    }

    let mut by_h = BTreeSet::new();
    let mut by_w = BTreeSet::new();

    for (i, &(h, w)) in items.iter().enumerate() {
        by_h.insert((h, w, i));
        by_w.insert((w, h, i));
    }

    let mut ans = vec![String::new(); n];
    let mut curr_h = h;
    let mut curr_w = w;
    while curr_h > 0 && curr_w > 0 {
        if let Some(&(h, w, i)) = by_h.last() {
            if h == curr_h {
                by_h.remove(&(h, w, i));
                by_w.remove(&(w, h, i));
                curr_w -= w;
                ans[i] = format!("1 {}", curr_w + 1);
                continue;
            }
        }
        
        if let Some(&(w, h, i)) = by_w.last() {
            if w == curr_w {
                by_h.remove(&(h, w, i));
                by_w.remove(&(w, h, i));
                curr_h -= h;
                ans[i] = format!("{} 1", curr_h + 1);
                continue;
            }
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
