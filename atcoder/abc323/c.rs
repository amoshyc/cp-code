#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    let mut curr = vec![0; n];
    let mut cands = vec![vec![]; n];
    for i in 0..n {
        let s = reads();
        curr[i] = i as i64 + 1;
        for j in 0..m {
            if s[j] == 'x' {
                cands[i].push(arr[j]);
            } else {
                curr[i] += arr[j];
            }
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let max = (0..n).filter(|&j| j != i).map(|j| curr[j]).max().unwrap();
        let mut need = max - curr[i];

        cands[i].sort();
        cands[i].reverse();

        for &x in cands[i].iter() {
            if need < 0 {
                break;
            } else {
                ans[i] += 1;
                need -= x;
            }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
