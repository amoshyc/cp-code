#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        asks: [(Usize1, Usize1); q],
    }

    // cmp_cw(i, j) = LT <-> i to j is clockwise
    // cmp_cw(i, j) = EQ <-> i, j has same angle
    // cmp_cw(i, j) = GT <-> i to j is counter-clockwise
    let cmp_cw = |i, j| {
        let (x0, y0) = xy[i];
        let (x1, y1) = xy[j];
        ((y0, x0) < (0, 0)) // quadrant of a
            .cmp(&((y1, x1) < (0, 0))) // quadrant of b
            .then_with(|| (x1 * y0).cmp(&(x0 * y1))) // slope(a), slope(b)
            .reverse() // ccw to cw
    };

    // sort points by clockwise
    let mut indices = (0..n).collect::<Vec<usize>>();
    indices.sort_by(|i, j| cmp_cw(*i, *j));

    let mut ans = vec![];
    for (a, b) in asks {
        if cmp_cw(a, b).is_le() {
            // a to b is clockwise, they are ordered in indices.
            let l = indices.partition_point(|&i| cmp_cw(i, a).is_lt());
            let r = indices.partition_point(|&i| cmp_cw(i, b).is_le());
            ans.push(r - l);
        } else {
            // a to b is couter-clockwise, their order is reversed.
            let l = indices.partition_point(|&i| cmp_cw(i, a).is_lt());
            let r = indices.partition_point(|&i| cmp_cw(i, b).is_le());
            ans.push(n + r - l);
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
