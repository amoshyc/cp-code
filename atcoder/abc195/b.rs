#![allow(unused)]

fn main() {
    let inp = readv::<i32>();
    let (a, b, w) = (inp[0], inp[1], inp[2] * 1000);

    let min = (w + b - 1) / b;
    let max = w / a;
    if min <= max {
        println!("{} {}", min, max);
    } else {
        println!("UNSATISFIABLE");
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
