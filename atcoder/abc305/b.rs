#![allow(unused)]

fn main() {
    let s = reads();
    let p = s[0];
    let q = s[2];

    let mut map = std::collections::HashMap::new();
    map.insert('A', 0i32);
    map.insert('B', 3);
    map.insert('C', 4);
    map.insert('D', 8);
    map.insert('E', 9);
    map.insert('F', 14);
    map.insert('G', 23);

    let x = map[&p];
    let y = map[&q];
    println!("{}", (x - y).abs());
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
