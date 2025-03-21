#![allow(unused)]

fn main() {
    let q = read::<usize>();
    let mut pref = vec![0];
    let mut head = 0;
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<i64>();
        if ask[0] == 1 {
            pref.push(pref[pref.len() - 1] + ask[1]);
        } else if ask[0] == 2 {
            head += 1;
        } else {
            let k = ask[1] as usize;
            ans.push(pref[head + k - 1] - pref[head]);
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
