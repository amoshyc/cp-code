#![allow(unused)]


fn main() {
    let inp = readv::<usize>();
    let (r, c) = (inp[0] - 1, inp[1] - 1);
    let mut arr = vec![];
    for i in 0..2 {
        arr.push(readv::<usize>());
    }
    println!("{}", arr[r][c]);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
