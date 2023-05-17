#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, x) = (inp[0] as usize, inp[1]);
    let mut arr = readv::<i64>();
    arr.sort();

    for i in 1..n {
        if let Ok(j) = arr.binary_search(&(x + arr[i])) {
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
