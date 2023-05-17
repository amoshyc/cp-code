#![allow(unused)]

fn main() {
    let mut s = vec![];
    for _ in 0..8 {
        s.push(reads());
    }

    for r in 0..8 {
        for c in 0..8 {
            if s[r][c] == '*' {
                println!("{}{}", (c as u8 + 'a' as u8) as char, (8 - r));
                return;
            }
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
