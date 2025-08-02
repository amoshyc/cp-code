#![allow(unused)]

fn main() {
    // -1 -1 1 1 should be yes

    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        let n = read::<usize>();
        let mut arr = readv::<i64>();

        // A[i + 1] = r * A[i]
        // A[i + 1] = s * (p / q) * A[i]
        // A[i + 1] * q = s * p * A[i]
        arr.sort_by_key(|x| x.abs());
        let s = arr[0].signum() * arr[1].signum();
        let p = arr[1].abs();
        let q = arr[0].abs();
        let mut ok = (1..n).all(|i| arr[i] * q == s * p * arr[i - 1]);

        // Only when r = -1 above method fails. 
        // Additionally check r = -1
        let cnt_pos = (0..n).filter(|&i| arr[i] == arr[0].abs()).count();
        let cnt_neg = (0..n).filter(|&i| arr[i] == -arr[0].abs()).count();
        if cnt_pos + cnt_neg == n && cnt_pos.abs_diff(cnt_neg) <= 1 {
            ok = true;
        }

        if ok {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
