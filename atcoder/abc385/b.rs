#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (h, w, sr, sc) = (inp[0], inp[1], inp[2] - 1, inp[3] - 1);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }
    let t = reads();

    let mut r = sr;
    let mut c = sc;
    let mut set = HashSet::new();
    let drdc = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let dirs = ['R', 'L', 'D', 'U'];
    for x in t {
        let (dr, dc) = drdc[dirs.iter().position(|d| *d == x).unwrap()];
        let nr = r.checked_add_signed(dr).unwrap_or(h);
        let nc = c.checked_add_signed(dc).unwrap_or(w);
        if nr < h && nc < w && arr[nr][nc] != '#' {
            (r, c) = (nr, nc);
            if arr[nr][nc] == '@' {
                set.insert((nr, nc));
            }
        }
    }

    println!("{} {} {}", r + 1, c + 1, set.len());
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
