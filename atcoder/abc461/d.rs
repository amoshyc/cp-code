#![allow(unused)]

use std::collections::HashMap;

use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i64,
        s: [Chars; h],
    }

    let mut arr = vec![vec![0 as i64; w]; h];
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '1' {
                arr[r][c] = 1;
            }
        }
    }

    let pref = build_2d(&arr);

    let mut ans = 0;
    for r1 in 0..h {
        for r2 in r1..h {
            let col_sums = (0..w).map(|c| query_2d(&pref, r1, r2, c, c)).collect();
            let cnt = solve_1d(col_sums, k);
            ans += cnt;
        }
    }

    println!("{ans}");
}

fn solve_1d(arr: Vec<i64>, k: i64) -> i64 {
    // find number of segments that has segment sum = k
    // pref[r] - pref[l - 1] = k or pref[r] = k

    let n = arr.len();
    let mut cnt = HashMap::new();
    let mut pref = 0;
    let mut ans = 0;

    for i in 0..n {
        pref += arr[i];

        ans += cnt.get(&(pref - k)).unwrap_or(&0);
        if pref == k {
            ans += 1;
        }

        *cnt.entry(pref).or_insert(0) += 1;
    }

    ans
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
