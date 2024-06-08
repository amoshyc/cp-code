#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<String>();
        let name = inp[0].clone();
        let height = inp[1].parse::<i32>().unwrap();
        arr.push((name, height));
    }
    arr.sort_by_key(|(n, h)| *h);
    println!("{}", arr[arr.len() - 2].0);
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
