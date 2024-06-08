#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(read::<String>());
    }
    arr.sort();
    arr.dedup();
    println!("{}", arr.len());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T1: std::str::FromStr, T2: std::str::FromStr>() -> (T1, T2) {
    let inp = read::<String>();
    let inp: Vec<_> = inp.split_ascii_whitespace().collect();
    let a: T1 = inp[0].parse().ok().unwrap();
    let b: T2 = inp[1].parse().ok().unwrap();
    (a, b)
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<_>()
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
