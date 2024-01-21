#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let s = reads();

    let mut r = 0;
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<i64>();
        let (t, x) = (inp[0], inp[1]);
        if t == 1 {
            r += x;
        } else {
            let x = x - 1;
            let i = (x - r).rem_euclid(n as i64) as usize;
            ans.push(s[i]);
        }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
