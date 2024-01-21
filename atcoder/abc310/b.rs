#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut prices = vec![];
    let mut functions = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        prices.push(inp[0]);
        let mut set: HashSet<usize> = HashSet::new();
        set.extend(inp[2..].iter());
        functions.push(set);
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let mut ok = true;
            ok &= prices[i] >= prices[j];
            ok &= functions[i].is_subset(&functions[j]);
            ok &= prices[i] > prices[j] || functions[j].len() > functions[i].len();

            if ok {
                println!("Yes");
                return;
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

fn mapv<T, F>(arr: &Vec<T>, f: fn(&T) -> F) -> Vec<F> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
