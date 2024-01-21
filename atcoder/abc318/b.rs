#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut vis = vec![vec![false; 101]; 101];
    for _ in 0..n {
        let inp = readv::<usize>();
        let (r1, r2, c1, c2) = (inp[0], inp[1], inp[2], inp[3]);
        for r in r1..r2 {
            for c in c1..c2 {
                vis[r][c] = true;
            }
        }
    }

    let mut ans = 0;
    for r in 0..=100 {
        for c in 0..=100 {
            if vis[r][c] {
                ans += 1;
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}