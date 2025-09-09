#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [Chars; n],
        ask: [(Usize1, Usize1, Usize1, Usize1); q],
    }

    let mut f = vec![vec![0; n]; n];
    for r in 0..(n - 1) {
        for c in 0..(n - 1) {
            if s[r + 1][c] == '.' && s[r][c + 1] == '.' && s[r + 1][c + 1] == '.' && s[r][c] == '.'
            {
                f[r][c] = 1;
            }
        }
    }

    let pref = build_2d(&f);

    let mut ans = vec![];
    for (r0, r1, c0, c1) in ask {
        ans.push(query_2d(&pref, r0, r1 - 1, c0, c1 - 1));
    }
    println!("{}", join(&ans, "\n"));
}

fn build_2d(arr: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (nr, nc) = (arr.len(), arr[0].len());
    let mut pref = vec![vec![0; nc]; nr];
    let transition = [((-1, 0), 1), ((0, -1), 1), ((-1, -1), -1)];
    for r in 0..nr {
        for c in 0..nc {
            pref[r][c] = arr[r][c];
            for &((dr, dc), s) in transition.iter() {
                let pr = r.checked_add_signed(dr).unwrap_or(nr);
                let py = c.checked_add_signed(dc).unwrap_or(nc);
                if pr < nr && py < nc {
                    pref[r][c] += s * pref[pr][py];
                }
            }
        }
    }
    pref
}

// arr[r1..=r2, c1..=c2]
fn query_2d(pref: &Vec<Vec<i64>>, r1: usize, r2: usize, c1: usize, c2: usize) -> i64 {
    let (nr, nc) = (pref.len(), pref[0].len());
    let r1 = r1.checked_add_signed(-1).unwrap_or(nr);
    let c1 = c1.checked_add_signed(-1).unwrap_or(nc);
    let transition = [((r2, c2), 1), ((r1, c2), -1), ((r2, c1), -1), ((r1, c1), 1)];
    let mut res = 0;
    for ((r, c), s) in transition {
        if r < nr && c < nc {
            res += s * pref[r][c];
        }
    }
    res
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
