#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut set = std::collections::BTreeMap::new();
    for i in 0..n {
        let inp = readv::<i64>();
        let (s, c) = (inp[0], inp[1]);
        set.insert(s, c);
    }

    let mut ans = 0;
    while let Some((s, c)) = set.pop_first() {
        if c >= 2 {
            *set.entry(2 * s).or_insert(0) += c / 2;
        }
        if c % 2 == 1 {
            ans += 1;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
