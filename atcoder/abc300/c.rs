#![allow(unused)]

use std::convert::TryFrom;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut ans = vec![0; h.min(w) + 1];

    for r in 0..h {
        for c in 0..w {
            if arr[r][c] != '#' {
                continue;
            }
            for s in (0..=h.min(w) as i32).rev() {
                let mut all = true;
                for d in 1..=s {
                    for &(dr, dc) in [(d, d), (-d, -d), (d, -d), (-d, d)].iter() {
                        let nr = usize::try_from((r as i32) + dr).unwrap_or(h);
                        let nc = usize::try_from((c as i32) + dc).unwrap_or(w);
                        if nr >= h || nc >= w || arr[nr][nc] != '#' {
                            all = false;
                        }
                    }
                }

                let mut any = false;
                for &(dr, dc) in [
                    (s + 1, s + 1),
                    (-(s + 1), -(s + 1)),
                    (s + 1, -(s + 1)),
                    (-(s + 1), s + 1),
                ]
                .iter()
                {
                    let nr = usize::try_from((r as i32) + dr).unwrap_or(h);
                    let nc = usize::try_from((c as i32) + dc).unwrap_or(w);
                    if nr >= h || nc >= w || arr[nr][nc] != '#' {
                        any = true;
                    }
                }

                if all && any {
                    ans[s as usize] += 1;
                    break;
                }
            }
        }
    }

    println!("{}", join(&ans[1..], " "));
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
