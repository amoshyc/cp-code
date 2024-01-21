#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, ccc) = (inp[0], inp[1] as i64);
    let mut points = std::collections::BTreeMap::new();
    for _ in 0..n {
        let inp = readv::<i64>();
        let (a, b, c) = (inp[0], inp[1], inp[2]);
        *points.entry(a).or_insert(0) += c;
        *points.entry(b + 1).or_insert(0) += -c;
    }

    let mut ans = 0;
    let mut p = 0;
    let mut c = 0;
    for (&d, &x) in points.iter() {
        ans += (d - p) * c.min(ccc);
        c += x;
        p = d;
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
