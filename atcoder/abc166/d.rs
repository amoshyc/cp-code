#![allow(unused)]

fn main() {
    // f(x) = x^5 是遞增的函數，所以兩個 x^5 相減最小值只發生在 f(x) - f(x - 1)，且 f(x) - f(x - 1) 也是遞增的。
    // f(x) - f(x - 1) >= 1e9 occurs at x >= 120 or x <= -120
    // 所以只需處理 x=-120..=120
    let k = read::<i64>();

    for x in -120i64..=120 {
        for y in -120i64..=120 {
            if x.pow(5) - y.pow(5) == k {
                println!("{} {}", x, y);
                return;
            }
        }
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
