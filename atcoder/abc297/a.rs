#![allow(unused)]

fn main() {
    let inp = readv::<i32>();
    let (n, d) = (inp[0] as usize, inp[1]);
    let arr = readv::<i32>();

    if let Some(w) = arr.windows(2).find(|&w| w[1] - w[0] <= d) {
        println!("{}", w[1]);
    } else {
        println!("-1");
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
