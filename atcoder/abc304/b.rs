#![allow(unused)]

fn main() {
    let n = read::<i32>();

    if n < 1_000 {
        println!("{}", n);
    } else {
        for i in 0..6u32 {
            if n < 10i32.pow(i + 4) {
                let base = 10i32.pow(i + 1);
                println!("{}", n / base * base);
                return;
            }
        }
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
