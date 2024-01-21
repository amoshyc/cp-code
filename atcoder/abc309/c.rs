#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1] as i64);
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = a.iter().sum::<i64>() + 10;
    let ok = |m: i64| {
        (0..n).filter(|&i| a[i] >= m).map(|i| b[i]).sum::<i64>() > k
    };

    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}