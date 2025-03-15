#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let m = 1_000_000 + 1;
    let mut pos = vec![vec![]; m];
    for i in 0..n {
        pos[arr[i]].push(i);
    }

    let mut ans = n + 1;
    for x in 0..m {
        if pos[x].len() >= 2 {
            for w in pos[x].windows(2) {
                ans = ans.min(w[1] - w[0] + 1);
            }
        }
    }

    if ans == n + 1 {
        println!("-1");
    } else {
        println!("{ans}");
    }
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
