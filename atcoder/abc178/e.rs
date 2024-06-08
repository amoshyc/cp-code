#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut u = vec![];
    let mut v = vec![];
    for _ in 0..n {
        let xy = readv::<i64>();
        u.push((xy[0] + xy[1]));
        v.push((xy[0] - xy[1]));
    }
    let ans1 = u.iter().max().unwrap() - u.iter().min().unwrap();
    let ans2 = v.iter().max().unwrap() - v.iter().min().unwrap();
    println!("{}", ans1.max(ans2));
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
