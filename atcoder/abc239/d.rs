#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (a, b) = (inp[0], inp[1]);
    let (c, d) = (inp[2], inp[3]);

    let is_prime = |x| {
        for i in (2..).take_while(|&i| i * i <= x) {
            if x % i == 0 {
                return false;
            }
        }
        true
    };

    for x in a..=b {
        if (c..=d).all(|y| !is_prime(x + y)) {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
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
