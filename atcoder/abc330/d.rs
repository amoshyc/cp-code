#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for i in (0..n) {
        arr.push(reads());
    }

    let mut cnt_r = vec![0 as i64; n];
    let mut cnt_c = vec![0 as i64; n];
    for r in 0..n {
        for c in 0..n {
            if arr[r][c] == 'o' {
                cnt_r[r] += 1;
                cnt_c[c] += 1;
            }
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..n {
            if arr[r][c] == 'o' {
                ans += (cnt_r[r] - 1) * (cnt_c[c] - 1);
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
