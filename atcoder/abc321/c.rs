#![allow(unused)]

fn main() {
    let mut ans = vec![];
    let mut que = std::collections::VecDeque::new();
    for i in 1..=9 {
        que.push_back(i as i64);
    }
    while let Some(x) = que.pop_front() {
        ans.push(x);
        for i in 0..(x % 10) {
            let y = x * 10 + i;
            que.push_back(y);
        }
    }
    let k = read::<usize>();
    println!("{}", ans[k - 1]);
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
