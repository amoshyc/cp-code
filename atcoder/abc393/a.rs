#![allow(unused)]

fn main() {
    let s = readv::<String>();
    let a = s[0].chars().next().unwrap();
    let b = s[1].chars().next().unwrap();
    match (a, b) {
        ('f', 'f') => println!("4"),
        ('s', 'f') => println!("2"),
        ('s', 's') => println!("1"),
        ('f', 's') => println!("3"),
        _ => (),
    };
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
