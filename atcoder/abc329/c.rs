#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut set = std::collections::HashMap::new();

    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && s[j] == s[i] {
            j += 1;
        }

        let entry = set.entry(s[i]).or_insert(0);
        *entry = (*entry).max(j - i);
        i = j;
    }

    let mut ans = 0;
    for (k, v) in set {
        ans += v;
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
