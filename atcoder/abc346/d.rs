#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();
    let cost = readv::<i64>();

    let mut f1 = vec![0; n]; // match with 01010101
    let mut f2 = vec![0; n]; // match with 10101010
    for i in 0..n {
        let exp = if i % 2 == 0 { '0' } else { '1' };
        if s[i] != exp {
            f1[i] = cost[i];
        }
        let exp = if i % 2 == 0 { '1' } else { '0' };
        if s[i] != exp {
            f2[i] = cost[i];
        }
    }
    let p1 = build(&f1);
    let p2 = build(&f2);

    let mut ans = 10i64.pow(18);

    // ...00...
    // 1_00_101
    // 01_00_101
    // 101_00_101
    // 0101_00_101
    for i in 0..(n - 1) {
        let mut mid = 0;
        if s[i] != '0' {
            mid += cost[i];
        }
        if s[i + 1] != '0' {
            mid += cost[i + 1];
        }
        let left = if i % 2 == 0 {
            query(&p1, 0, i)
        } else {
            query(&p2, 0, i)
        };
        let right = if i % 2 == 0 {
            query(&p2, i + 2, n)
        } else {
            query(&p1, i + 2, n)
        };
        ans = ans.min(mid + left + right);
    }

    // ...11...
    for i in 0..(n - 1) {
        let mut mid = 0;
        if s[i] != '1' {
            mid += cost[i];
        }
        if s[i + 1] != '1' {
            mid += cost[i + 1];
        }
        let left = if i % 2 == 0 {
            query(&p2, 0, i)
        } else {
            query(&p1, 0, i)
        };
        let right = if i % 2 == 0 {
            query(&p1, i + 2, n)
        } else {
            query(&p2, i + 2, n)
        };
        ans = ans.min(mid + left + right);
    }

    println!("{}", ans);
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
