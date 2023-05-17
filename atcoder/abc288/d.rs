#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![vec![0; n]; k];
    for r in 0..k {
        for i in 0..n {
            pref[r][i] = if i > 0 { pref[r][i - 1] } else { 0 };
            if i % k == r {
                pref[r][i] += arr[i];
            }
        }
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (a, b) = (inp[0] - 1, inp[1] - 1);

        let mut sum = vec![0; k];
        for r in 0..k {
            sum[r] = pref[r][b] - if a == 0 { 0 } else { pref[r][a - 1] };
        }
        sum.sort();
        sum.dedup();

        if sum.len() == 1 {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
