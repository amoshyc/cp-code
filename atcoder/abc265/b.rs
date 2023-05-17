#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, t) = (inp[0], inp[1], inp[2]);
    let mut arr = readv::<usize>();
    arr.insert(0, 0);

    let mut bonus = vec![0; n + 1];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (x, y) = (inp[0], inp[1]);
        bonus[x] += y;
    }

    let mut ans = true;
    let mut time = t;
    for pos in 1..n {
        time += bonus[pos];
        if time > arr[pos] {
            time -= arr[pos];
            continue;
        } else {
            ans = false;
            break;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
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
