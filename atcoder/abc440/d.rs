#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut arr: [i64; n],
        ask: [(i64, i64); q],
    }

    arr.sort();

    let mut ans = vec![];
    for (x, y) in ask {
        let x_idx = arr.partition_point(|a| *a < x);

        // 0 0 0 1 1 1
        let ok = |m: i64| -> bool {
            let cnt = arr.partition_point(|a| *a <= m) - x_idx;
            x + cnt as i64 + (y - 1) <= m
        };

        let mut lb = x;
        let mut ub = x + y + n as i64 + 1;
        if ok(lb) {
            ans.push(lb);
            continue;
        }

        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if ok(m) {
                ub = m;
            } else {
                lb = m;
            }
        }
        ans.push(ub);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
