#![allow(unused)]

use std::collections::{BTreeSet, HashMap};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut xs: [i64; n],
    }

    let mut nearest = HashMap::new();
    let mut set = BTreeSet::new();

    let inf = 10i64.pow(10);
    set.insert(0);
    nearest.insert(0, inf);

    let mut ans = vec![];
    let mut sum = inf;

    for x in xs {
        // ..., a, x, b, ...
        let a = *set.range(..x).last().unwrap_or(&-1);
        let b = *set.range(x..).next().unwrap_or(&-1);

        // Update a
        if a != -1 {
            let old_dis = a.abs_diff(nearest[&a]) as i64;
            if x - a < old_dis {
                sum -= old_dis;
                sum += x - a;
                nearest.insert(a, x);
            }
        }

        // Update b
        if b != -1 {
            let old_dis = b.abs_diff(nearest[&b]) as i64;
            if b - x < old_dis {
                sum -= old_dis;
                sum += b - x;
                nearest.insert(b, x);
            }
        }

        // Insert x
        set.insert(x);
        if b == -1 || ((a >= 0 && b >= 0) && x - a < b - x) {
            sum += x - a;
            nearest.insert(x, a);
        } else {
            sum += b - x;
            nearest.insert(x, b);
        }

        ans.push(sum);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
