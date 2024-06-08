#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut s = (0, 0);
    let mut t = (0, 0);
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'S' {
                s = (r, c);
                arr[r][c] = '.';
            } else if arr[r][c] == 'T' {
                t = (r, c);
                arr[r][c] = '.';
            }
        }
    }

    let n = read::<usize>();
    let mut med = vec![vec![0; w]; h];
    for _ in 0..n {
        let inp = readv::<usize>();
        let (r, c, e) = (inp[0] - 1, inp[1] - 1, inp[2] as i32);
        med[r][c] = e;
    }

    let mut rem = vec![vec![-1; w]; h];
    let mut que = VecDeque::new();

    if med[s.0][s.1] > 0 {
        rem[s.0][s.1] = med[s.0][s.1];
        que.push_back(s);
    }

    while let Some((r, c)) = que.pop_front() {
        if rem[r][c] == 0 {
            continue;
        }
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(h);
            let nc = c.checked_add_signed(dc).unwrap_or(w);
            if nr < h && nc < w && arr[nr][nc] == '.' {
                let new = (rem[r][c] - 1).max(med[nr][nc]);
                if new > rem[nr][nc] {
                    rem[nr][nc] = new;
                    que.push_back((nr, nc));
                }
            }
        }
    }

    if rem[t.0][t.1] >= 0 {
        println!("Yes");
    } else {
        println!("No");
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
