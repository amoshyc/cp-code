#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut res = arr.clone();
    res[0].rotate_right(1);
    res[n - 1].rotate_left(1);
    for r in 0..(n - 1) {
        res[r][0] = arr[r + 1][0];
    }
    for r in 1..n {
        res[r][n - 1] = arr[r - 1][n - 1];
    }

    let mut ans = vec![];
    for r in 0..n {
        ans.push(join(&res[r], ""));
    }
    println!("{}", join(&ans, "\n"));
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