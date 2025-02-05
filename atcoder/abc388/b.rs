#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, d) = (inp[0], inp[1]);
    let mut t = vec![];
    let mut l = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        t.push(inp[0]);
        l.push(inp[1]);
    }

    for k in 1..=d {
        let mut max = 0;
        for i in 0..n {
            max = max.max(t[i] * (l[i] + k));
        }
        println!("{}", max);
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
