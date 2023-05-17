#![allow(unused)]

fn main() {
    let inp = readv::<u128>();
    let (n, m) = (inp[0], inp[1]);

    if n * n < m {
        println!("-1");
        return;
    }
    if m == 1 {
        println!("1");
        return;
    }

    let mut x = 1;
    while x * x < m {
        x += 1;
    }

    let mut ans = std::u128::MAX;
    for i in 1..=x {
        let q = (m + (i - 1)) / i;
        if q <= n {
            ans = ans.min(i * q);
        }
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
