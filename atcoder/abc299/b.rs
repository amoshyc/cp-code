#![allow(unused)]


fn main() {
    let inp = readv::<i64>();
    let (n, t) = (inp[0] as usize, inp[1]);
    let c = readv::<i64>();
    let r = readv::<i64>();

    let b = if c.contains(&t) { t } else { c[0] };
    let ans = (0..n).filter(|&i| c[i] == b).max_by_key(|&i| r[i]).unwrap();
    println!("{}", ans + 1);
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
