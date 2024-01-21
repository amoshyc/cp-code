#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, x) = (inp[0], inp[1] as i64);
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    let mut pref_a = a[0];
    let mut pref_b = b[0];
    let mut ans = a[0] + x * b[0];
    for i in 1..n {
        pref_a += a[i];
        pref_b += b[i];
        ans = ans.min(pref_a + pref_b + b[i] * (x - 1 - i as i64).max(0));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
