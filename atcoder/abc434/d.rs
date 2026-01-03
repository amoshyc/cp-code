#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        boxes: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    // Answer of cloud i is `base + cnt1` where
    //     base: the total number of nonzero cell
    //     cnt1: the number of cells in cloud i that is covered exactly once
    // Both can be found in O(H * W) using diff 2d & prefix 2d.

    let (h, w) = (2_000, 2_000);

    let mut diff = vec![vec![0; w + 1]; h + 1];
    for (i, &(r0, r1, c0, c1)) in boxes.iter().enumerate() {
        diff[r0][c0] += 1;
        diff[r0][c1 + 1] -= 1;
        diff[r1 + 1][c0] -= 1;
        diff[r1 + 1][c1 + 1] += 1;
    }

    let cnt = build_2d(&diff);

    let mut is1 = vec![vec![0; w]; h];
    let mut base = 0;
    for r in 0..h {
        for c in 0..w {
            if cnt[r][c] == 1 {
                is1[r][c] = 1;
            }
            if cnt[r][c] == 0 {
                base += 1;
            }
        }
    }

    let pref = build_2d(&is1);
    let ans = boxes
        .iter()
        .map(|&(r0, r1, c0, c1)| base + query_2d(&pref, r0, r1, c0, c1))
        .collect::<Vec<_>>();

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
