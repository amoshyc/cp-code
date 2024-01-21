#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, p, q) = (inp[0], inp[1], inp[2]);
    let d = *readv::<usize>().iter().min().unwrap();
    println!("{}", p.min(q + d));
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
