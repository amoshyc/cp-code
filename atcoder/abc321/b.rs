#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, x) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    for y in 0..=100 {
        let mut s = arr.clone();
        s.push(y);
        s.sort();
        let total = s[1..n - 1].iter().sum::<usize>();
        if total >= x {
            println!("{}", y);
            return;
        }
    }
    println!("-1");
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
