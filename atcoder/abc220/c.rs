#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<u64>();
    let x = read::<u64>();

    let mut pref = vec![arr[0]];
    for i in 1..n {
        pref.push(pref[i - 1] + arr[i]);
    }

    let q = x / pref[n - 1];
    let r = x % pref[n - 1];
    let mut ans = q * n as u64;

    for i in 0..n {
        if pref[i] > r {
            ans += (i + 1) as u64;
            break;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
