#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let mut faces = vec![HashSet::new(); n];
    let mut prob = vec![vec![0.0; 100_001]; n];
    for i in 0..n {
        let arr = readv::<usize>();
        let k = arr[0] as f64;
        for &x in arr.iter().skip(1) {
            faces[i].insert(x);
            prob[i][x] += 1.0 / k;
        }
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut val = 0.0;
            for &f in faces[i].iter() {
                val += prob[i][f] * prob[j][f];
            }
            ans = ans.max(val);
        }
    }
    println!("{:.9}", ans);
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
