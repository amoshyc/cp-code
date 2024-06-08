#![allow(unused)]

use std::io;
use std::io::Write;

fn main() {
    let n = read::<usize>();
    let m = n.next_power_of_two().trailing_zeros() as usize;
    println!("{}", m);

    let mut juices = vec![];
    for i in 0..m {
        let bottles: Vec<usize> = (0..n).filter(|j| (j & (1 << i)) != 0).collect();
        let bottles = mapv(&bottles, |x| x + 1);
        juices.push(format!("{} {}", bottles.len(), join(&bottles, " ")));
    }
    println!("{}", join(&juices, "\n"));
    io::stdout().flush();

    let res = reads();
    let mut ans = 0;
    for i in 0..m {
        if res[i] == '1' {
            ans |= (1 << i);
        }
    }

    println!("{}", ans + 1);
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
