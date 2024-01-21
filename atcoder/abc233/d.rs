#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![arr[0]];
    for i in 1..n {
        pref.push(pref[i - 1] + arr[i]);
    }

    let mut ans = pref.iter().filter(|&x| *x == k).count();
    let mut map = std::collections::HashMap::new();
    for &p in pref.iter() {
        ans += map.get(&(p - k)).unwrap_or(&0);
        *map.entry(p).or_insert(0) += 1;
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
