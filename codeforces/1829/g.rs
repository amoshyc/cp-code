#![allow(unused)]

fn f(x: u64) -> u64 {
    x * (x + 1) * (2 * x + 1) / 6
}

fn solve() {
    let n = read::<u64>();
    let mut head = vec![0; 1_001];
    let mut tail = vec![0; 1_001];

    let mut curr = 0;
    for i in 1..=1500 {
        let head = 1 + i * (i - 1) / 2;
        let tail = i * (i + 1) / 2;
        if head <= n && n <= tail {
            curr = i;
            break;
        }
    }

    let mut cnt = 0;
    let mut lb = n;
    let mut ub = n;
    for l in (1..=curr).rev() {
        cnt += f(ub) - f(lb - 1);
        // println!("{}: {} {}, {}", l, lb, ub, f(ub) - f(lb - 1));
        if lb == 1 {
            break;
        }
        lb = (lb - l).max(1 + (l - 1) * (l - 2) / 2);
        ub = (ub + 1 - l).min((l - 1) * l / 2);
    }
    println!("{}", cnt);
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
    }
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
