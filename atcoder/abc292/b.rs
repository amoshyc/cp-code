#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut cnt = vec![0; n];

    for _ in 0..q {
        let inp = readv::<usize>();
        let (cmd, x) = (inp[0], inp[1] - 1);

        if cmd == 1 {
            cnt[x] += 1;
        } else if cmd == 2 {
            cnt[x] += 2;
        } else {
            println!("{}", if cnt[x] >= 2 { "Yes" } else { "No" });
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
