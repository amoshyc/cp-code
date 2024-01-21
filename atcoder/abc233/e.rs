#![allow(unused)]

fn main() {
    let s = reads();
    let x: Vec<u32> = s.iter().map(|c| c.to_digit(10).unwrap()).collect();
    let n = x.len();

    let mut pref = vec![x[0]];
    for i in 1..n {
        pref.push(pref[i - 1] + x[i]);
    }

    let mut carry = 0;
    let mut ans = vec![];
    for i in 0..n {
        let sum = pref[n - 1 - i] + carry;
        ans.push(sum % 10);
        carry = sum / 10;
    }
    while carry > 0 {
        ans.push(carry % 10);
        carry /= 10;
    }

    ans.reverse();
    println!("{}", join(&ans, ""));
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
