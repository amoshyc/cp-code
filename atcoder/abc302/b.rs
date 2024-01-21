#![allow(unused)]

use std::convert::TryFrom;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let s = "snuke".chars().collect::<Vec<char>>();
    for r in 0..n {
        for c in 0..m {
            for &(dr, dc) in [
                (1, 1),
                (0, 1),
                (1, 0),
                (-1, -1),
                (0, -1),
                (-1, 0),
                (1, -1),
                (-1, 1),
            ]
            .iter()
            {
                let mut pos = vec![];
                for i in 0..5 {
                    let nr = usize::try_from((r as i32) + i * dr).unwrap_or(!0);
                    let nc = usize::try_from((c as i32) + i * dc).unwrap_or(!0);
                    if nr < n && nc < m && arr[nr][nc] == s[i as usize] {
                        pos.push(format!("{} {}", nr + 1, nc + 1));
                    }
                }
                if pos.len() == 5 {
                    println!("{}", join(&pos, "\n"));
                }
            }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
