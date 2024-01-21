#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut arr = vec![vec![1i64; n]; n];
    for r in 1..n {
        for c in 1..n {
            arr[r][c] = arr[r - 1][c] + arr[r][c - 1];
        }
    }

    for r in 0..n {
        let mut ans = vec![];
        for i in 0..=r {
            ans.push(arr[r - i][i]);
        }
        println!("{}", join(&ans, " "));
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
