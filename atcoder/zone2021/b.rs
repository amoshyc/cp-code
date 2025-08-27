#![allow(unused)]

fn main() {
    // (x0, y0), (x1, y1)
    // y = ax + b
    // y0 = a x0 + b
    // y1 = a x1 + b
    // y1 - y0 = a (x1 - x0)
    // a = (y1 - y0) / (x1 - x0)
    // b = y1 - a x1

    let inp = readv::<usize>();
    let (n, x1, y1) = (inp[0], inp[1] as f64, inp[2] as f64);
    let mut ans: f64 = 0.0;
    for _ in 0..n {
        let inp = readv::<f64>();
        let (x0, y0) = (inp[0], inp[1]);
        let a = (y1 - y0) / (x1 - x0);
        let b = y1 - a * x1;
        ans = ans.max(b);
    }
    println!("{:.4}", ans);
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
