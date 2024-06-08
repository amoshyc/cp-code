#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    let ok = |mut x: i64| {
        for i in (0..n).rev() {
            x -= arr[i];
            if x < 0 {
                return false;
            }
        }
        true
    };

    // 0 0 0 1 1 1
    let mut lb = 0i64;
    let mut ub = 10i64.pow(15);
    if ok(lb) {
        println!("{}", lb);
        return;
    }
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
