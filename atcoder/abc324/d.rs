#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = read::<u64>();

    let stat = |mut x: u64| {
        let mut cnt = vec![0; 10];
        let mut k = 0;
        while x > 0 {
            cnt[(x % 10) as usize] += 1;
            x /= 10;
            k += 1;
        }
        if k < n {
            cnt[0] += n - k;
        }
        cnt
    };

    let max = 10u64.pow(n as u32);
    let stat_s = stat(s);

    let mut ans = 0;
    for i in (0..).take_while(|i| i * i <= max) {
        if stat(i * i) == stat_s {
            ans += 1;
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
