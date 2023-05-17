#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (n, a, b) = (inp[0] as usize, inp[1], inp[2]);
    let arr = reads();

    let mut ans = std::u64::MAX;
    for i in 0..n {
        let mut x = arr.clone();
        x.rotate_left(i);
        let cnt = (0..n).filter(|&i| x[i] != x[n - 1 - i]).count() / 2;
        let cost = (i as u64) * a + (cnt as u64) * b;
        ans = ans.min(cost);
    }

    println!("{}", ans);
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
