#![allow(unused)]

fn solve() {
    let n = read::<usize>();
    let mut ans = n;
    for x in (0..=n).take_while(|x| 2 * x <= n) {
        let y = (n - 2 * x) / 4;
        if 2 * x + 4 * y == n {
            ans = ans.min(x + y);
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
