#![allow(unused)]

fn main() {
    let mut arr = readv::<usize>();
    arr.sort();

    let ok1 = arr[0] == arr[1] && arr[1] == arr[2] && arr[2] != arr[3] && arr[3] == arr[4];
    let ok2 = arr[0] == arr[1] && arr[1] != arr[2] && arr[2] == arr[3] && arr[3] == arr[4];

    if ok1 || ok2 {
        println!("Yes");
    } else {
        println!("No");
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