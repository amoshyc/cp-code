#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, p, q, r, s) = (inp[0], inp[1] - 1, inp[2] - 1, inp[3] - 1, inp[4] - 1);
    let arr = readv::<usize>();

    let res = [
        &arr[..p],
        &arr[r..=s],
        &arr[q + 1..r],
        &arr[p..=q],
        &arr[s + 1..],
    ]
    .concat();

    println!("{}", join(&res, " "));
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
