#![allow(unused)]

fn solve() {
    let s = reads();
    let mut h = (s[0] as u32 - '0' as u32) * 10 + (s[1] as u32 - '0' as u32);
    if h >= 12 {
        h -= 12;
        if h == 0 {
            h = 12;
        }
        println!("{:02}:{}{} PM", h, s[3], s[4]);
    } else {
        if h == 0 {
            h = 12;
        }
        println!("{:02}:{}{} AM", h, s[3], s[4]);
    }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
