#![allow(unused)]


fn main() {
    let s1 = reads();
    let s2 = reads();
    let (a, b) = (s1[0], s1[1]);
    let (c, d) = (s2[0], s2[1]);

    if (a, b, c, d) == ('#', '.', '.', '#') {
        println!("No");
    } else if (a, b, c, d) == ('.', '#', '#', '.') {
        println!("No");
    } else {
        println!("Yes");
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
