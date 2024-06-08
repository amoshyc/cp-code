#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<usize>();
    let (h, w, m) = (inp[0], inp[1], inp[2]);
    let mut ask = vec![];
    for r in 0..h {
        ask.push((1, r, 0));
    }
    for _ in 0..m {
        let inp = readv::<usize>();
        ask.push((inp[0], inp[1] - 1, inp[2]));
    }

    let mut cnt_r = h;
    let mut cnt_c = w;
    let mut vis_r = vec![false; h];
    let mut vis_c = vec![false; w];

    let mut ans = HashMap::new();
    for &(t, i, x) in ask.iter().rev() {
        if t == 2 {
            if vis_c[i] {
                continue;
            }
            *ans.entry(x).or_insert(0) += cnt_r;
            cnt_c -= 1;
            vis_c[i] = true;
        } else {
            if vis_r[i] {
                continue;
            }
            *ans.entry(x).or_insert(0) += cnt_c;
            cnt_r -= 1;
            vis_r[i] = true;
        }
    }

    let mut ans: Vec<_> = ans.iter().filter(|&(k, v)| *v > 0).collect();
    ans.sort();
    
    println!("{}", ans.len());
    for (k, v) in ans {
        println!("{} {}", k, v);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
