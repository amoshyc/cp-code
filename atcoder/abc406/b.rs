#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1] as u32);
    let arr = readv::<i64>();

    let mut x: i64 = 1;
    for a in arr {
        x = x.saturating_mul(a);
        if x >= 10i64.pow(k) {
            x = 1;
        }
    }
    println!("{}", x);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
