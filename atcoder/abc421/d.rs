#![allow(unused)]

use std::collections::HashMap;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        mut rt: i64,
        mut ct: i64,
        mut ra: i64,
        mut ca: i64,
        n: usize,
        m: usize,
        l: usize,
        rle_t: [(char, i64); m],
        rle_a: [(char, i64); l],
    }

    // Split rle to same length actions
    let mut rle_t = VecDeque::from(rle_t);
    let mut rle_a = VecDeque::from(rle_a);
    let dirs = HashMap::from([('R', (0, 1)), ('L', (0, -1)), ('U', (-1, 0)), ('D', (1, 0))]);
    let mut actions = vec![];
    while let Some(((char_t, step_t), (char_a, step_a))) = rle_t.pop_front().zip(rle_a.pop_front()) {
        let min = step_t.min(step_a);
        actions.push((dirs[&char_t], dirs[&char_a], min));
        if min != step_t {
            rle_t.push_front((char_t, step_t - min));
        }
        if min != step_a {
            rle_a.push_front((char_a, step_a - min));
        }
    }

    // For each action (dir_t, dir_a, step), find the number of k such that
    // 1 <= k <= step
    // (rt, ct) + k * dir_t = (ra, ca) + k * dir_a
    // (rt, ct) - (ra, ca) = k * (dir_a - dir_t)

    let mut ans = 0;
    for (dir_t, dir_a, step) in actions {
        if dir_a == dir_t {
            if (rt, ct) == (ra, ca) {
                ans += step;
            }
        } else if dir_a.0 == dir_t.0 {
            // one is L, other is R
            if let Some(k) = full_div(ct - ca, dir_a.1 - dir_t.1) {
                if 1 <= k && k <= step && ra == rt {
                    ans += 1;
                }
            }
        } else if dir_a.1 == dir_t.1 {
            // one is U, other is D
            if let Some(k) = full_div(rt - ra, dir_a.0 - dir_t.0) {
                if 1 <= k && k <= step && ca == ct {
                    ans += 1;
                }
            }
        } else {
            // orthogonal
            let k_r = full_div(rt - ra, dir_a.0 - dir_t.0);
            let k_c = full_div(ct - ca, dir_a.1 - dir_t.1);
            if let Some((k_r, k_c)) = k_r.zip(k_c) {
                if k_r == k_c && 1 <= k_r && k_r <= step {
                    ans += 1;
                }
            }
        }
        rt += step * dir_t.0;
        ct += step * dir_t.1;
        ra += step * dir_a.0;
        ca += step * dir_a.1;
    }

    println!("{ans}");
}

fn full_div(a: i64, b: i64) -> Option<i64> {
    if a % b != 0 {
        None
    } else {
        Some(a / b)
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
