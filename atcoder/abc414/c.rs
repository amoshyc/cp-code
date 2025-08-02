#![allow(unused)]

fn main() {
    let base = read::<i64>();
    let n = read::<i64>();

    let mut ans = 0;

    let ok = |x: i64| -> bool {
        if x < 1 || x > n {
            return false;
        }
        let digits = extract_digits(x, base);
        let m = digits.len();
        (0..m).all(|i| digits[i] == digits[m - 1 - i])
    };

    for pref in 0..=1_000_000 {
        let digits = extract_digits(pref, 10);
        let m = digits.len();

        // abc cba
        let mut even = 0;
        for i in 0..m {
            even = even * 10 + digits[i];
        }
        for i in (0..m).rev() {
            even = even * 10 + digits[i];
        }
        if ok(even) {
            ans += even;
        }

        // abc d cba
        let mut odd = 0;
        for i in 0..digits.len() {
            odd = odd * 10 + digits[i];
        }
        odd = odd * 10;
        for i in (0..digits.len()).rev() {
            odd = odd * 10 + digits[i];
        }
        for center in 0..10 {
            let val = odd + center * 10i64.pow(m as u32);
            if ok(val) {
                ans += val;
            }
        }
    }

    println!("{}", ans);
}

fn extract_digits(mut x: i64, base: i64) -> Vec<i64> {
    let mut digits = vec![];
    while x > 0 {
        digits.push(x % base);
        x /= base;
    }
    digits.reverse();
    digits
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
