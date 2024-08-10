#![allow(unused)]

fn main() {
    let y = read::<usize>();
    if is_leap(y) {
        println!("366");
    } else {
        println!("365");
    }
}

fn is_leap(y: usize) -> bool {
    if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else if y % 4 == 0 {
        true
    } else {
        false
    }
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
