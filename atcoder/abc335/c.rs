#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut history = std::collections::VecDeque::new();
    for i in 1..=n {
        history.push_back((i as i32, 0 as i32));
    }

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<String>();
        if inp[0] == "1" {
            let (x, y) = history[0];
            if inp[1] == "R" {
                history.push_front((x + 1, y));
            } else if inp[1] == "L" {
                history.push_front((x - 1, y));
            } else if inp[1] == "U" {
                history.push_front((x, y + 1));
            } else {
                history.push_front((x, y - 1));
            }
        } else {
            let p = inp[1].parse::<usize>().unwrap();
            let (x, y) = history[p - 1];
            ans.push(format!("{} {}", x, y));
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
