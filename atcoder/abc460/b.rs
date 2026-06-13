#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    for _ in 0..tc {
        input! {
            x1: i64,
            y1: i64,
            r1: i64,
            x2: i64,
            y2: i64,
            r2: i64,
        }

        // dis <= r1 + r2
        let d2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);

        let max_r = r1.max(r2);
        let min_r = r1.min(r2);

        let mut touch = true;
        // d < R - r
        if d2 < (max_r - min_r) * (max_r - min_r) {
            touch = false;
        }
        // d > R + r
        if d2 > (max_r + min_r) * (max_r + min_r) {
            touch = false;
        }

        if touch {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
