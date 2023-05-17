#![allow(unused)]


fn main() {
    let n = read::<usize>();
    let mut arr = readv::<usize>();
    arr.insert(0, 1_000_000);

    let mut cnt1 = 0 as u64;
    for i in 1..=n {
        if arr[i] == i {
            cnt1 += 1;
        }
    }
    
    let mut cnt2 = 0 as u64;
    for i in 1..=n {
        if arr[i] != i {
            if arr[arr[i]] == i {
                cnt2 += 1;
            }
        }
    }
    
    let ans = cnt1 * (cnt1 - 1) / 2 + cnt2 / 2;
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
