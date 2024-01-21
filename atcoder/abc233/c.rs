#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, x) = (inp[0] as usize, inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        arr.push(inp[1..].to_vec());
    }

    println!("{}", dfs(0, &arr, x));
}

fn dfs(i: usize, arr: &Vec<Vec<i64>>, x: i64) -> i64 {
    let n = arr.len();
    if i == n - 1 {
        return arr[n - 1].iter().filter(|&y| *y == x).count() as i64;
    }

    let mut cnt = 0;
    for &y in arr[i].iter() {
        if x % y == 0 {
            cnt += dfs(i + 1, arr, x / y);
        }
    }

    cnt
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
