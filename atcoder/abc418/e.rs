#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut groups = HashMap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (xy[j].0 - xy[i].0);
            let dy = (xy[j].1 - xy[i].1);
            let d2 = dx * dx + dy * dy;

            let g = gcd(dx.abs(), dy.abs());
            let (dx, dy) = (dx / g, dy / g);

            // normalize (dx, dy) in regard to (-dx, -dy)
            let pos = (dx, dy);
            let neg = (-dx, -dy);
            let (dx, dy) = if pos < neg { pos } else { neg };

            groups.entry((dx, dy)).or_insert(vec![]).push(d2);
        }
    }

    // Trapezoid (opposite side has different length) will be counted twice.

    // Parallelogram will be counted four times.
    // parallelogram <-> length of opposite side is the same.

    let mut abcd = 0; // opposite side has different length
    let mut abab = 0; // opposite side has same length <-> parallelogram
    for (&k, v) in &groups {
        if v.len() >= 2 {
            let mut cnt = HashMap::new();
            for &l in v {
                *cnt.entry(l).or_insert(0) += 1;
            }

            for &l in v {
                abcd += v.len() as i64 - cnt[&l];
                abab += cnt[&l] - 1;
            }
        }
    }

    let ans = abcd / 2 + abab / 4;
    println!("{ans}");
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
