#![allow(unused)]

fn solve() {
    let inp = readv::<i64>();
    let (n, x) = (inp[0], inp[1]);
    let mut ans = 0i64;
    for a in (1..).take_while(|a| a * 1 + a * 1 + 1 <= n && a + 1 + 1 <= x) {
        for b in (1..).take_while(|b| a * b + a * 1 + b * 1 <= n && a + b + 1 <= x) {
            let c1 = (n - a * b) / (a + b);
            let c2 = x - a - b;
            ans += c1.min(c2);
        }
    }
    println!("{}", ans);
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
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
