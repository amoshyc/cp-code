#![allow(unused)]

use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let inp = readv::<i64>();
    let (n, m) = (inp[0] as usize, inp[1] as usize);
    let (mut sx, mut sy) = (inp[2], inp[3]);

    let mut xs = HashMap::new();
    let mut ys = HashMap::new();
    for id in 0..n {
        let xy = readv::<i64>();
        let (x, y) = (xy[0], xy[1]);
        xs.entry(x).or_insert(BTreeSet::new()).insert((y, id));
        ys.entry(y).or_insert(BTreeSet::new()).insert((x, id));
    }

    let mut ans = HashSet::new();
    for _ in 0..m {
        let inp = readv::<String>();
        let dir = inp[0].chars().next().unwrap();
        let c: i64 = inp[1].parse().unwrap();

        let mut to_remove = vec![];

        if dir == 'U' {
            if xs.contains_key(&sx) {
                for (y, id) in xs[&sx].range((sy, 0)..=(sy + c, n)) {
                    ans.insert((sx, *y));
                    to_remove.push((sx, *y, *id));
                }
            }
            (sx, sy) = (sx, sy + c);
        } else if dir == 'D' {
            if xs.contains_key(&sx) {
                for (y, id) in xs[&sx].range((sy - c, 0)..=(sy, n)) {
                    ans.insert((sx, *y));
                    to_remove.push((sx, *y, *id));
                }
            }
            (sx, sy) = (sx, sy - c);
        } else if dir == 'L' {
            if ys.contains_key(&sy) {
                for (x, id) in ys[&sy].range((sx - c, 0)..=(sx, n)) {
                    ans.insert((*x, sy));
                    to_remove.push((*x, sy, *id));
                }
            }
            (sx, sy) = (sx - c, sy);
        } else {
            if ys.contains_key(&sy) {
                for (x, id) in ys[&sy].range((sx, 0)..=(sx + c, n)) {
                    ans.insert((*x, sy));
                    to_remove.push((*x, sy, *id));
                }
            }
            (sx, sy) = (sx + c, sy);
        }

        for (x, y, id) in to_remove {
            xs.get_mut(&x).unwrap().remove(&(y, id));
            ys.get_mut(&y).unwrap().remove(&(x, id));
        }
    }

    println!("{} {} {}", sx, sy, ans.len());
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
