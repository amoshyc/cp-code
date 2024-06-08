#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let m = 998_244_353;
    let arr = readv::<i64>();

    let mut pow10 = vec![];
    for i in 0..11 {
        pow10.push(10i64.pow(i) % m);
    }

    let num_digit = |mut x: i64| {
        let mut cnt = 1;
        while x >= 10 {
            cnt += 1;
            x /= 10;
        }
        cnt
    };

    let mut pref = 0;
    let mut ans = 0;
    for i in 0..n {
        let val1 = pref % m * pow10[num_digit(arr[i])] % m;
        let val2 = arr[i] * (i as i64) % m;
        ans += val1;
        ans %= m;
        ans += val2;
        ans %= m;
        pref += arr[i];
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
    read::<String>().chars().collect::<_>()
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
