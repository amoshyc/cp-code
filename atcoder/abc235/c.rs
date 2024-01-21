#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut pos = std::collections::HashMap::new();
    for (i, &a) in arr.iter().enumerate() {
        pos.entry(a).or_insert(vec![]).push(i);
    }

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (x, k) = (inp[0], inp[1] - 1);
        if pos.contains_key(&x) && k < pos[&x].len() {
            ans.push(format!("{}", pos[&x][k] + 1));
        } else {
            ans.push("-1".to_string());
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
