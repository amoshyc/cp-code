#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut cnt = std::collections::HashMap::new();
    for _ in 0..n {
        let s = read::<String>();
        *cnt.entry(s).or_insert(0) += 1;
    }

    let max = cnt.values().max().unwrap();
    for (k, v) in cnt.iter() {
        if v == max {
            println!("{}", k);
            break;
        }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
