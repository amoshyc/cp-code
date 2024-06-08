#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut arr = vec![];
    for _ in 0..3 {
        arr.push(readv::<i64>());
    }
    let mut pos = vec![vec![2; 3]; 3];
    let mut dp = HashMap::new();

    if dfs(&mut pos, 0, &arr, &mut dp) == 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn dfs(
    pos: &mut Vec<Vec<usize>>,
    who: usize,
    arr: &Vec<Vec<i64>>,
    dp: &mut HashMap<Vec<Vec<usize>>, usize>,
) -> usize {
    if dp.contains_key(pos) {
        return dp[pos];
    }

    let mut win = 2;
    for r in 0..3 {
        if pos[r][0] != 2 && (0..3).all(|c| pos[r][c] == pos[r][0]) {
            win = pos[r][0];
        }
    }
    for c in 0..3 {
        if pos[0][c] != 2 && (0..3).all(|r| pos[r][c] == pos[0][c]) {
            win = pos[0][c];
        }
    }
    if pos[0][0] != 2 && (0..3).all(|i| pos[i][i] == pos[0][0]) {
        win = pos[0][0];
    }
    if pos[2][0] != 2 && (0..3).all(|i| pos[2 - i][i] == pos[2][0]) {
        win = pos[2][0];
    }
    if win != 2 {
        dp.insert(pos.clone(), win);
        return win;
    }

    let mut whites = vec![];
    let mut score_t = 0;
    let mut score_a = 0;
    for r in 0..3 {
        for c in 0..3 {
            if pos[r][c] == 2 {
                whites.push((r, c));
            } else if pos[r][c] == 0 {
                score_t += arr[r][c];
            } else {
                score_a += arr[r][c];
            }
        }
    }

    if whites.len() == 0 {
        let win = if score_t > score_a { 0 } else { 1 };
        dp.insert(pos.clone(), win);
        return win;
    }

    let mut any = false;
    for (r, c) in whites {
        pos[r][c] = who;
        any |= dfs(pos, 1 - who, arr, dp) == who;
        pos[r][c] = 2;
    }

    if any {
        dp.insert(pos.clone(), who);
        who
    } else {
        dp.insert(pos.clone(), 1 - who);
        1 - who
    }
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
    read::<String>().chars().collect::<_>()
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
