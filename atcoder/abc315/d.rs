#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        let inp = reads();
        let row = mapv(&inp, |&x| (x as u32 - 'a' as u32) as usize);
        arr.push(row);
    }

    let mut rows = HashMap::new();
    let mut cols = HashMap::new();

    for r in 0..h {
        rows.insert(r, vec![0; 26]);
    }
    for c in 0..w {
        cols.insert(c, vec![0; 26]);
    }

    for r in 0..h {
        for c in 0..w {
            rows.entry(r).and_modify(|e| e[arr[r][c]] += 1);
            cols.entry(c).and_modify(|e| e[arr[r][c]] += 1);
        }
    }

    for _ in 0..(h + w) {
        let mut del_r = vec![];
        let mut del_c = vec![];

        for (r, cnts) in rows.iter() {
            for i in 0..26 {
                if cnts[i] > 1 && cnts[i] == cols.len() {
                    del_r.push((*r, i));
                }
            }
        }

        for (c, cnts) in cols.iter() {
            for i in 0..26 {
                if cnts[i] > 1 && cnts[i] == rows.len() {
                    del_c.push((*c, i));
                }
            }
        }

        for (r, i) in del_r {
            rows.remove(&r);
            for c in 0..w {
                cols.entry(c).and_modify(|e| e[i] -= 1);
            }
        }
        for (c, i) in del_c {
            cols.remove(&c);
            for r in 0..h {
                rows.entry(r).and_modify(|e| e[i] -= 1);
            }
        }
    }

    println!("{}", rows.len() * cols.len());
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
