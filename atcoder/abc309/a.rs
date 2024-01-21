#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (a, b) = (inp[0], inp[1]);

    let cands = vec![(1, 2), (2, 3), (4, 5), (5, 6), (7, 8), (8, 9)];
    for &(c, d) in cands.iter() {
        if c == a.min(b) && d == a.max(b) {
            println!("Yes");
            return;
        }
    }
    println!("No");
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