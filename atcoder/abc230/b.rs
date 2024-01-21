#![allow(unused)]

fn main() {
    let mut t = vec![];
    for _ in 0..10 {
        t.push('o');
        t.push('x');
        t.push('x');
    }

    let t = join(&t, "");
    let s = read::<String>();

    if t.contains(&s) {
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
