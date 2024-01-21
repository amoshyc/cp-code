#![allow(unused)]


fn main() {
    let inp = readv::<u64>();
    let mut a = inp[0];
    let mut b = inp[1];

    let mut ans = false;
    for _ in 0..18 {
        if (a % 10) + (b % 10) >= 10 {
            ans = true;
        }
        a /= 10;
        b /= 10;
    }

    if ans {
        println!("Hard");
    } else {
        println!("Easy");
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
