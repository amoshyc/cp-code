#![allow(unused)]

fn main() {
    let n = read::<i64>();

    let ok = |m: i64| -> bool {
        // 1 + 2 + ... + m >= n
        (1 + m) * m / 2 >= n
    };

    // 0 0 0 1 1 1
    let mut lb = 1;
    let mut ub = n;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }
    println!("{}", ub);
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
