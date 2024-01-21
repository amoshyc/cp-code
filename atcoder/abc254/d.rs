#![allow(unused)]

fn main() {
    let n = read::<usize>();

    // f(x) = largest divisor of x that is square number
    // (a * b) is square number if and only if a / f(a) = b / f(b)

    let mut f = vec![0; n + 1];
    for d in (1..).take_while(|&d| d * d <= n) {
        for x in ((d * d)..=n).step_by(d * d) {
            f[x] = d * d;
        }
    }

    let mut cnt = std::collections::HashMap::new();
    for i in 1..=n {
        *cnt.entry(i / f[i]).or_insert(0) += 1_i64;
    }

    let mut ans = 0;
    for &v in cnt.values() {
        ans += v * v;
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
