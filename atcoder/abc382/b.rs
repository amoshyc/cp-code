#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, d) = (inp[0], inp[1]);
    let mut s = reads();

    let mut pos = vec![];
    for i in 0..n {
        if s[i] == '@' {
            pos.push(i);
        }
    }

    for _ in 0..d {
        let p = pos.pop().unwrap();
        s[p] = '.';
    }

    println!("{}", join(&s, ""));
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
