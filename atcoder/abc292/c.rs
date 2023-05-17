#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut ans = 0;

    let mut cnt = vec![0 as u64; n + 1];
    // cnt[i] = number of (a, b) such that a * b = i
    for a in 1..=n {
        for b in (1..).take_while(|b| a * b <= n) {
            cnt[(a * b) as usize] += 1;
        }
    }

    for ab in (1..).take_while(|ab| ab + 1 <= n) {
        ans += cnt[ab] * cnt[(n - ab)];
    }

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
