#![allow(unused)]

fn solve() -> usize {
    let n = read::<usize>();
    let s = reads();
    for l in 1..=(n / 2) {
        if n % l != 0 {
            continue;
        }
        for p in [&s[..l], &s[l..(2 * l)]] {
            let mut cnt = 0;
            for i in 0..n {
                if s[i] != p[i % l] {
                    cnt += 1;
                }
            }
            if cnt <= 1 {
                return l;
            }
        }
    }

    n
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
    }
    println!("{}", join(&ans, "\n"));
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
