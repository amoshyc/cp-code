#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, s, m, l) = (inp[0], inp[1], inp[2], inp[3]);

    let mut ans = 10i64.pow(18);
    for a in (0..=n) {
        for b in (0..=n) {
            for c in (0..=n) {
                let total = a * 6 + b * 8 + c * 12;
                let cost = a * s + b * m + c * l;
                // println!("{} {} {} {} {}", a, b, c, total, cost);
                if total >= n {
                    ans = ans.min(cost);
                }
            }
        }
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
