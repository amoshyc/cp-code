#![allow(unused)]

fn main() {
    // 1 ~ 3 digit: 0
    // 4 ~ 6 digit: 1
    // 7 ~ 9 digit: 2
    let n = read::<i64>();
    let mut v = n;
    let mut k = 0;
    while v > 0 {
        k += 1;
        v /= 10;
    }

    let mut ans = 0;
    for i in 1..k {
        let cnt = 10i64.pow(i) - 10i64.pow(i - 1);
        ans += cnt * ((i as i64 - 1) / 3);
    }
    ans += (n - 10i64.pow(k - 1) + 1) * ((k as i64 - 1) / 3);

    println!("{}", ans);
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
