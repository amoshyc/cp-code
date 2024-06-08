#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut groups_u = vec![vec![]; 4];
    let mut groups_v = vec![vec![]; 4];
    for _ in 0..n {
        let inp = readv::<i64>();
        let u = inp[0] + inp[1];
        let v = inp[0] - inp[1];
        let gid = u.abs() % 2 * 2 + v.abs() % 2;
        groups_u[gid as usize].push(u);
        groups_v[gid as usize].push(v);
    }

    let mut ans = 0;
    for i in 0..4 {
        ans += f(groups_u[i].clone()) / 2;
        ans += f(groups_v[i].clone()) / 2;
    }
    println!("{}", ans);
}

fn f(mut arr: Vec<i64>) -> i64 {
    if arr.len() == 0 {
        return 0;
    }

    arr.sort();
    let mut ans = 0;
    let mut sum = arr[0];
    for i in 1..arr.len() {
        ans += i as i64 * arr[i] - sum;
        sum += arr[i];
    }

    ans
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
