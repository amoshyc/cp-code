#![allow(unused)]

// [Problem]
// There are N rectangles of paper on a two-dimensional plane. All the papers are arranged so that their edges are parallel to the x-axis or the y-axis.
// The coordinates of the bottom-left corner of the i-th paper are (lxi, lyi), and the coordinates of the top-right corner are (rxi, ryi).
// For k = 1,2,3,...,N, please find the following value:
// The area where exactly k papers overlap.

// [Solution]
// Difference Array, but in 2D.
// Add a rect r1..=r2, c1..=c2 in Diff2D:
//     Add +1 at (r1, r1)
//     Add -1 at (r1, c2 + 1)
//     Add -1 at (r2 + 1, c1)
//     Add +1 at (r2 + 1, c2 + 1)
// After constructing the Diff2D, the count is restored via:
//     cnt[r, c] = sum(diff[0..=r][0..=c])
//               = cnt[r - 1, c] + cnt[r, c - 1] - cnt[r - 1, c - 1] + diff[r][c]

fn main() {
    let n = read::<usize>();
    let mut diff = vec![vec![0 as i32; 1111]; 1111];
    for _ in 0..n {
        let rect = readv::<usize>();
        let (r1, c1, r2, c2) = (rect[0], rect[1], rect[2] - 1, rect[3] - 1);
        diff[r1][c1] += 1;
        diff[r1][c2 + 1] -= 1;
        diff[r2 + 1][c1] -= 1;
        diff[r2 + 1][c2 + 1] += 1;
    }

    let cnt = build_2d(&diff);

    let mut ans = vec![0; n + 1];
    for r in 0..1111 {
        for c in 0..1111 {
            ans[cnt[r][c] as usize] += 1;
        }
    }
    println!("{}", join(&ans[1..], "\n"));
}

fn build_2d<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Default + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    assert!(arr.len() >= 1 && arr[0].len() >= 1);
    let (n, m) = (arr.len(), arr[0].len());
    let mut pref = vec![vec![T::default(); m]; n];
    for r in 0..n {
        for c in 0..m {
            pref[r][c] = arr[r][c];
            if r >= 1 {
                pref[r][c] = pref[r][c] + pref[r - 1][c];
            }
            if c >= 1 {
                pref[r][c] = pref[r][c] + pref[r][c - 1];
            }
            if r >= 1 && c >= 1 {
                pref[r][c] = pref[r][c] - pref[r - 1][c - 1];
            }
        }
    }
    pref
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
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
