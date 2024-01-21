#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    let mut a = vec![];
    for i in 0..n {
        let inp = readv::<String>();
        let age = inp[1].parse::<i32>().unwrap();
        s.push(inp[0].clone());
        a.push(age);
    }

    let j = (0..n).min_by_key(|&i| a[i]).unwrap();
    for i in 0..n {
        println!("{}", s[(j + i) % n]);
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
