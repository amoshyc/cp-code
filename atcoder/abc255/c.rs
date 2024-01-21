#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (x, a, d, n) = (inp[0], inp[1], inp[2], inp[3]);

    if d == 0 {
        println!("{}", (x - a).abs());
        return;
    }

    let t1 = (x - a) / d;
    let t2 = (x - a + d - 1) / d;
    let mut ans = std::i64::MAX;
    for t in vec![t1, t2] {
        let t = t.max(0).min(n - 1);
        let y = a + t * d;
        ans = ans.min((y - x).abs());
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
    read::<String>().chars().collect::<Vec<char>>()
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
