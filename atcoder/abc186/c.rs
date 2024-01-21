#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut ans = 0;
    for i in 1..=n {
        let v1 = digits(i, 10);
        let v2 = digits(i, 8);
        if !v1.contains(&7) && !v2.contains(&7) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn digits(mut x: usize, b: usize) -> Vec<usize> {
    let mut res = vec![];
    while x > 0 {
        res.push(x % b);
        x /= b;
    }
    res
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
