#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pos: [i64; n],
    }

    pos.sort();

    let ok = |m: i64| -> bool { pos.windows(2).all(|w| w[1] - w[0] <= m) };

    // 0 0 0 1 1 1
    let mut lb = -1;
    let mut ub = 10i64.pow(10);
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }

    println!("{ub}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
