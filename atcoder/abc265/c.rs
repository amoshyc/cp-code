#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<i32>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut vis = HashSet::new();
    let mut r = 0 as i32;
    let mut c = 0 as i32;
    vis.insert((r, c));
    loop {
        let mut nr = r;
        let mut nc = c;
        match arr[r as usize][c as usize] {
            'U' => nr = r - 1,
            'D' => nr = r + 1,
            'L' => nc = c - 1,
            'R' => nc = c + 1,
            _ => ()
        };
        if nr < 0 || nr >= n || nc < 0 || nc >= m{
            println!("{} {}", r + 1, c + 1);
            break;
        } else if vis.contains(&(nr, nc)) {
            println!("-1");
            break;
        } else {
            vis.insert((nr, nc));
            r = nr;
            c = nc;
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}
