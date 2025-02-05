#![allow(unused)]

fn main() {
    let n = read::<i64>();
    let x = readv::<i64>();

    let avg = x.iter().sum::<i64>();
    let mut ans = i64::MAX;
    for y in [avg / n, (avg + (n - 1)) / n] {
        let val = x.iter().map(|&x| (x - y).pow(2)).sum::<i64>();
        ans = ans.min(val);
    }
    println!("{}", ans);
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
