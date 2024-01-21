#![allow(unused)]

fn main() {
    let x = read::<i64>();

    let mut l = 0;
    let mut v = x;
    while v > 0 {
        l += 1;
        v /= 10;
    }

    let mut ans: i64 = 1_111_111_111_111_111_111;
    for &i in [l, l + 1].iter() {
        for s in 1..=9 {
            for d in (-9)..=9 {
                let mut digits = vec![s];
                for j in 0..(i - 1) {
                    digits.push(digits[j] + d);
                }
                if digits.iter().all(|&y| 0 <= y && y <= 9) {
                    let mut y = digits[0];
                    for i in 1..i {
                        y = y * 10 + digits[i];
                    }
                    if y >= x {
                        ans = ans.min(y);
                    }
                }
            }
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
