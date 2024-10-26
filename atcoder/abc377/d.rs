#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    // Count the complement, the number of interval that contains any of the given interval.
    let mut inv = vec![vec![]; m];
    for _ in 0..n {
        let inp = readv::<usize>();
        let l = inp[0] - 1;
        let r = inp[1] - 1;
        inv[r].push(l);
    }

    let mut max_l = -1;
    let mut cnt = 0;
    for r in 0..m {
        for l in inv[r].iter() {
            max_l = max_l.max(*l as i64);
        }
        cnt += max_l + 1;
    }

    let ans = (m as i64) * (m as i64 + 1) / 2 - cnt;
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
    read::<String>().chars().collect()
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
