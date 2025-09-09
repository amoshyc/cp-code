#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        pts: [(i64, i64); n],
    }

    let mut rand = XorShift64 { seed: 123 };

    for _ in 0..200 {
        let i = (rand.new() % (n as u64)) as usize;
        let j = (rand.new() % (n as u64)) as usize;
        if i == j {
            continue;
        }
        let cnt = pts
            .iter()
            .filter(|&&pt| are_collinear(pts[i], pts[j], pt))
            .count();
        if 2 * cnt >= n {
            let (a, b, c) = line_from_two_point(pts[i], pts[j]);
            println!("Yes");
            println!("{} {} {}", a, b, c);
            return;
        }
    }

    println!("No");
}

// y - y1 = (dy / dx) x - x1 where dx = x1 - x2, y1 = y1 - y2
// dy x - dx y + (dx y1 - dy x1) = 0
// (a, b, c) is not normalized by their gcd
fn line_from_two_point(pt1: (i64, i64), pt2: (i64, i64)) -> (i64, i64, i64) {
    let (dx, dy) = ((pt1.0 - pt2.0), (pt1.1 - pt2.1));
    let a = dy;
    let b = -dx;
    let c = dx * pt1.1 - dy * pt1.0;
    (a, b, c)
}

// slope is same
// (y2 - y1) / (x2 - x1) = (y3 - y1) / (x3 - x1)
fn are_collinear(pt1: (i64, i64), pt2: (i64, i64), pt3: (i64, i64)) -> bool {
    let ((x1, y1), (x2, y2), (x3, y3)) = (pt1, pt2, pt3);
    (y2 - y1) * (x3 - x1) == (y3 - y1) * (x2 - x1)
}

struct XorShift64 {
    seed: u64, // should be nonzero
}

impl XorShift64 {
    fn new(&mut self) -> u64 {
        assert!(self.seed != 0);
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
