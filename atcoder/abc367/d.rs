#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![0];
    for i in 0..n {
        pref.push(pref[pref.len() - 1] + arr[i]);
    }
    for i in 0..(n - 1) {
        pref.push(pref[pref.len() - 1] + arr[i]);
    }

    let mut groups = vec![vec![]; m];
    for i in 0..pref.len() {
        let r = (pref[i] % (m as i64)) as usize;
        groups[r].push(i);
    }

    let mut ans = 0;
    for r in 0..m {
        for i in 0..groups[r].len() {
            if groups[r][i] < n {
                let p = groups[r].partition_point(|x| *x <= groups[r][i] + n - 1);
                ans += (p - i - 1) as i64;
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
