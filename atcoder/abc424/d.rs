#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        tc: usize,
    }
    let mut out = vec![];
    for _ in 0..tc {
        input! {
            h: usize,
            w: usize,
            mut arr: [Chars; h],
        }

        let mut ans = usize::MAX;
        dfs(&mut arr, 0, &mut ans);
        out.push(ans);
    }
    println!("{}", join(&out, "\n"));
}

fn dfs(arr: &mut Vec<Vec<char>>, cnt: usize, ans: &mut usize) {
    if cnt >= *ans {
        return;
    }

    let h = arr.len();
    let w = arr[0].len();

    // Find a black 2x2 grid
    let mut grid = None;
    for r in 0..(h - 1) {
        for c in 0..(w - 1) {
            if arr[r][c] == '#'
                && arr[r][c + 1] == '#'
                && arr[r + 1][c] == '#'
                && arr[r + 1][c + 1] == '#'
            {
                grid = Some((r, c));
                break;
            }
        }
    }

    if let Some((r, c)) = grid {
        // Try to flip different position
        for (dr, dc) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            let (nr, nc) = (r + dr, c + dc);
            arr[nr][nc] = '.';
            dfs(arr, cnt + 1, ans);
            arr[nr][nc] = '#';
        }
    } else {
        *ans = (*ans).min(cnt);
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
