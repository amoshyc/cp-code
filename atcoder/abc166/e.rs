#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();
    let mut cnt1 = std::collections::HashMap::new();
    let mut cnt2 = std::collections::HashMap::new();
    for i in 0..n {
        *cnt1.entry(-(i as i32) - arr[i]).or_insert(0) += 1;
        *cnt2.entry((i as i32) - arr[i]).or_insert(0) += 1;
    }

    let mut ans = 0 as i64;
    for i in 0..n {
        ans += cnt1.get(&(arr[i] - i as i32)).unwrap_or(&0);
        ans += cnt2.get(&(arr[i] + i as i32)).unwrap_or(&0);
    }

    println!("{}", ans / 2);
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
