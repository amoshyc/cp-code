#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let cnt_t = s.iter().filter(|&&c| c == 'T').count();
    let cnt_a = s.len() - cnt_t;
    if cnt_t > cnt_a {
        println!("T");
    } else if cnt_t < cnt_a {
        println!("A");
    } else {
        println!("{}", if s[n - 1] == 'T' { 'A' } else { 'T' });
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
