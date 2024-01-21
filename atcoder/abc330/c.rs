#![allow(unused)]

fn main() {
    let d = read::<i64>();

    let mut ys = vec![];
    for y in (0..).take_while(|y| y * y <= d) {
        ys.push(y);
    }

    let mut ans = d;
    for x in (0..).take_while(|x| x * x < d) {
        let idx = ys.partition_point(|&y| x * x + y * y < d);
        if idx < ys.len() {
            let y = ys[idx];
            ans = ans.min((x * x + y * y - d).abs());
        }
        if idx >= 1 {
            let y = ys[idx - 1];
            ans = ans.min((x * x + y * y - d).abs());
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
