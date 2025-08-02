#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![0; tc];
    for tid in 0..tc {
        let inp = readv::<usize>();
        let (mut h, mut w) = (inp[0], inp[1]);
        let mut arr = vec![];
        for _ in 0..h {
            let row = reads();
            let row = mapv(&row, |&c| if c == '#' { 1 } else { -1 });
            arr.push(row);
        }

        // Make h smaller than w
        // -> h <= sqrt(h*w) = 547
        if h > w {
            arr = rotate_cw(&arr);
            (h, w) = (w, h);
        }

        let pref_2d = build_2d(&arr);

        let mut cnt = vec![0i64; 2 * h * w + 1];
        let shift = (h * w) as i64;

        for u in 0..h {
            for d in u..h {
                // Count pairs (l, r) such that sum(A[u..=d, l..=r]) = 0
                // `count_pairs_with_range_sum` is too slow due to HashMap.
                // We use shifted array `cnt` & `shift` to store frequency instead.
                // `cnt` is shared among different (u, d) combinations due to its size.
                // Therefore we need to restore it.

                // prefix sum of the column sum
                // pref[c] = sum(A[u..=d, 0..=c])
                let pref: Vec<i64> = (0..w).map(|c| query_2d(&pref_2d, u, d, 0, c)).collect();

                // sum(column_sum[l..=r]) = x
                // <->
                // 1. P[r] - P[l - 1] = x or
                // 2. P[r] = x
                // where P is prefix sum

                for i in 0..w {
                    let idx = (pref[i] + shift) as usize;
                    ans[tid] += cnt[idx]; // P[r] - P[l - 1] = 0
                    cnt[idx] += 1;
                }
                ans[tid] += cnt[(0 + shift) as usize]; // P[r] = 0

                // Restore cnt to 0
                for i in 0..w {
                    let idx = (pref[i] + shift) as usize;
                    cnt[idx] -= 1;
                }
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn count_pairs_with_range_sum(arr: &Vec<i64>, s: i64) -> i64 {
    let mut ans = 0;
    let mut cnt = HashMap::new();
    let mut pref = 0;
    for x in arr {
        pref += x;
        ans += cnt.get(&(pref - s)).unwrap_or(&0);
        *cnt.entry(pref).or_insert(0) += 1;
    }
    ans += cnt.get(&s).unwrap_or(&0);
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

// clockwise
//  123        41
//  456   ->   52
//             63
// (2x3)      (3x2)
fn rotate_cw<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![arr[0][0].clone(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[c][n - 1 - r] = arr[r][c].clone();
        }
    }
    res
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
