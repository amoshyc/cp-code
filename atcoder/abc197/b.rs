#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let (sr, sc) = (inp[2] - 1, inp[3] - 1);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let d = (sr + 1..h).take_while(|r| arr[*r][sc] == '.').count();
    let u = (0..sr).rev().take_while(|r| arr[*r][sc] == '.').count();
    let r = (sc + 1..w).take_while(|c| arr[sr][*c] == '.').count();
    let l = (0..sc).rev().take_while(|c| arr[sr][*c] == '.').count();
    let cnt = d + u + r + l + 1;
    println!("{}", cnt);
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
