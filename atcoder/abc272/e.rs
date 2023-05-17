#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    let mut sets = vec![std::collections::HashSet::new(); m + 1];
    for (i, &a) in arr.iter().enumerate() {
        // 0 <= a + (i + 1) * j <= n
        // 0 <= a + k * j => j >= ceil(-a / k)
        // a + k * j <= n => j <= floor((n - a) / k)
        let k = (i as i64) + 1;
        let j_min = ((-a + k - 1) / k).max(1);
        let j_max = ((n as i64 - a) / k).min(m as i64);
        for j in j_min..=j_max {
            sets[j as usize].insert(a + k * j);
        }
    }

    let mut ans = vec![];
    for i in 1..=m {
        let res = (0..=(n as i64)).find(|&x| !sets[i].contains(&x)).unwrap();
        ans.push(res);
    }
    println!("{}", join(&ans, "\n"));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}