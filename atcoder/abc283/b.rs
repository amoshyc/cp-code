#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<u32>();
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<u32>();
        if inp[0] == 1 {
            let (k, x) = ((inp[1] - 1) as usize, inp[2]);
            arr[k] = x;
        } else {
            let k = (inp[1] - 1) as usize;
            println!("{}", arr[k]);
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
