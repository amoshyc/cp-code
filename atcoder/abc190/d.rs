#![allow(unused)]

fn main() {
    let n = read::<i64>();

    let mut ans = 0;
    for l in (1..).take_while(|l| l * l <= 2 * n) {
        if (2 * n) % l == 0 {
            let q = 2 * n / l;
            if (q + 1 - l) % 2 == 0 {
                ans += 2;                
            }
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
