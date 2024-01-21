#![allow(unused)]


fn main() {
    let g1 = vec!["R G B", "G B R", "B R G"];
    let g2 = vec!["R B G", "B G R", "G R B"];

    let s = read::<String>();
    let t = read::<String>();
    let s = s.as_str();
    let t = t.as_str();

    if (g1.contains(&s) && g1.contains(&t)) || (g2.contains(&s) && g2.contains(&t)) {
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
