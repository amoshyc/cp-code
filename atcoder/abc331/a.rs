#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (a, b) = (inp[0], inp[1]);

    let inp = readv::<usize>();
    let mut y = inp[0];
    let mut m = inp[1];
    let mut d = inp[2];
    d += 1;
    if d == b + 1 {
        d = 1;
        m += 1;
        if m == a + 1 {
            m = 1;
            y += 1;
        }
    }
    println!("{} {} {}", y, m, d);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
