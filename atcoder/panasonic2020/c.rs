#![allow(unused)]

fn main() {
    // sqrt(a) + sqrt(b) < sqrt(c)

    // a + b + 2 * sqrt(ab) < c
    // 2 * sqrt(ab) < c - a - b
    // 4 * ab < (c - a - b)^2

    let inp = readv::<i64>();
    let (a, b, c) = (inp[0], inp[1], inp[2]);
    if 4 * a * b < (c - a - b).pow(2) && c >= a + b {
        println!("Yes");
    } else {
        println!("No");
    }
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
