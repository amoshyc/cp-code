#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut cnt_f = 0;
    let mut cnt_a = 0;
    for _ in 0..n {
        let s = reads();
        if s[0] == 'F' {
            cnt_f += 1;
        } else {
            cnt_a += 1;
        }
    }
    if cnt_f > cnt_a {
        println!("Yes");
    } else {
        println!("No");
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
