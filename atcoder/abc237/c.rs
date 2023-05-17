#![allow(unused)]

fn main() {
    let arr = reads();
    let n = arr.len();

    // aak
    // akkaa

    let cnt0 = (0..n).take_while(|&i| arr[i] == 'a').count();
    let cnt1 = (0..n).take_while(|&i| arr[n - 1 - i] == 'a').count();

    if cnt0 > cnt1 {
        println!("No");
        return;
    }

    let e = n - (cnt1 - cnt0);
    if (0..e).all(|i| arr[i] == arr[e - 1 - i]) {
        println!("Yes");
    } else {
        println!("No");
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
