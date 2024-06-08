#![allow(unused)]

const MOD: i64 = 998_244_353;

fn main() {
    //  index : 4 3 2 1 0
    //        : - - - - -
    //      0 : 0 0 0 0 0
    //      1 : 0 0 0 0 1
    //      2 : 0 0 0 1 0
    //      3 : 0 0 0 1 1
    //      4 : 0 0 1 0 0
    //      5 : 0 0 1 0 1
    //      6 : 0 0 1 1 0
    //      7 : 0 0 1 1 1
    //      8 : 0 1 0 0 0
    //      9 : 0 1 0 0 1
    //     10 : 0 1 0 1 0
    //     11 : 0 1 0 1 1
    //     12 : 0 1 1 0 0
    //     13 : 0 1 1 0 1  <-- n
    //     14 : 0 1 1 1 0
    //     15 : 0 1 1 1 1
    //     16 : 1 0 0 0 0
    // There are n + 1 numbers from 0..=n
    // i-th bit:
    //    * period = 2 ** (i + 1)
    //    * 0..(2 ** i) is 0
    //    * (2 ** i)..(2 ** (i + 1)) is 1

    let inp = readv::<i64>();
    let (n, m) = (inp[0], inp[1]);

    let mut ans = 0;
    for i in 0..60 {
        if (m >> i) & 1 == 1 {
            let period = 1 << (i + 1);
            let q = (n + 1) / period;
            let r = (n + 1) % period;
            ans += q * (1 << i);
            ans %= MOD;
            ans += (r - (1 << i)).max(0);
            ans %= MOD;
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
    read::<String>().chars().collect()
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
