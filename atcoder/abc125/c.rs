#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<u64>();

    let mut pref = vec![arr[0]; n];
    for i in 1..n {
        pref[i] = gcd(pref[i - 1], arr[i]);
    }

    let mut suff = vec![arr[n - 1]; n];
    for i in (0..(n - 1)).rev() {
        suff[i] = gcd(suff[i + 1], arr[i]);
    }

    let mut ans = 0;
    ans = ans.max(pref[n - 2]);
    ans = ans.max(suff[1]);
    for i in 1..(n - 1) {
        ans = ans.max(gcd(pref[i - 1], suff[i + 1]));
    }
    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
