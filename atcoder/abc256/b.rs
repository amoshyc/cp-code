#![allow(unused)]

fn main() {
    let n: usize = read();
    let a: Vec<usize> = readv();

    let mut ans = 0;
    let mut suff = 0;
    for i in (0..n).rev() {
        suff = suff + a[i];
        if suff >= 4 {
            ans += 1;
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
