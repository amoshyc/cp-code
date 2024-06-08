#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0], inp[1]);
    let m = 10i64.pow(9) + 7;

    let mut ans = 0;
    for i in k..=(n + 1) {
        // let min = (0..i).sum::<i64>();
        // let max = ((n - i + 1)..=n).sum::<i64>();
        // ans += max - min + 1;
        let min = ((i - 1) + 0) * i / 2 % m;
        let max = (n + (n - i + 1)) * i / 2 % m;
        ans += (max + m - min + 1) % m;
        ans %= m;
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
    read::<String>().chars().collect::<_>()
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
