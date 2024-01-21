#![allow(unused)]

use std::convert::TryFrom;

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    for r in (0..n) {
        for c in (0..n) {
            for &(dr, dc) in [(0, 1), (1, 0), (1, 1), (1, -1)].iter() {
                let mut cnt1 = 0;
                let mut cnt2 = 0;
                for i in 0..6 {
                    let nr = usize::try_from(r as i32 + i * dr).unwrap_or(n);
                    let nc = usize::try_from(c as i32 + i * dc).unwrap_or(n);
                    if nr < n && nc < n {
                        if arr[nr][nc] == '.' {
                            cnt1 += 1;
                        } else {
                            cnt2 += 1;
                        }
                    }
                }
                if cnt1 + cnt2 == 6 && cnt1 <= 2 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
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
