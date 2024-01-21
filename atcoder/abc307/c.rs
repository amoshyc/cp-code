#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let (ha, wa, a) = readarr();
    let (hb, wb, b) = readarr();
    let (hx, wx, x) = readarr();

    for dr in (-10)..=10 {
        for dc in (-10)..=10 {
            let ta: HashSet<_> = a.iter().map(|&(r, c)| (r + dr, c + dc)).collect();
            let c: HashSet<_> = ta.union(&b).cloned().collect();
            if norm(&c) == x {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn norm(pos: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut min_r = 10i32.pow(9);
    let mut min_c = 10i32.pow(9);
    for &(r, c) in pos.iter() {
        min_r = min_r.min(r);
        min_c = min_c.min(c);
    }
    let mut set = HashSet::new();
    for &(r, c) in pos.iter() {
        set.insert((r - min_r, c - min_c));
    }
    set
}

fn readarr() -> (usize, usize, HashSet<(i32, i32)>) {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut set = HashSet::new();
    for r in 0..h {
        let inp = reads();
        for c in 0..w {
            if inp[c] == '#' {
                set.insert((r as i32, c as i32));
            }
        }
    }
    (h, w, norm(&set))
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
