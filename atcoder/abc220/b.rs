#![allow(unused)]

fn main() {
    let k = read::<u32>();
    let inp = readv::<String>();
    let a: Vec<u32> = inp[0].chars().map(|c| c.to_digit(10).unwrap()).collect();
    let b: Vec<u32> = inp[1].chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut x = 0 as u64;
    let mut y = 0 as u64;
    for &a in a.iter() {
        x = x * k as u64 + a as u64;
    }
    for &b in b.iter() {
        y = y * k as u64 + b as u64;
    }

    println!("{}", x * y);
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
