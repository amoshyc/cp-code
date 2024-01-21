#![allow(unused)]

fn main() {
    let mut s = vec![];

    for l in 1..=10 {
        let mut arr = vec![11; l];
        dfs(0, &mut arr, &mut s);
    }
    s.sort();

    let k = read::<usize>();
    println!("{}", s[k - 1]);
}

fn dfs(i: usize, arr: &mut Vec<i64>, s: &mut Vec<i64>) {
    if i == arr.len() {
        let mut v = 0;
        for i in 0..arr.len() {
            v = v * 10 + arr[i];
        }
        s.push(v);
        return;
    }

    if i == 0 {
        for j in 1..=9 {
            arr[i] = j;
            dfs(i + 1, arr, s);
        }
    } else {
        for j in 0..=9 {
            if (arr[i - 1] - j).abs() <= 1 {
                arr[i] = j;
                dfs(i + 1, arr, s);
            }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
