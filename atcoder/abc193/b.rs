#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut ans = !0;
    for _ in 0..n {
        let inp = readv::<usize>();
        let (a, p, x) = (inp[0], inp[1], inp[2]);
        if x >= a + 1 {
            ans = ans.min(p);
        }
    }
    if ans == !0 {
        println!("-1");
    } else {
        println!("{}", ans);
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
