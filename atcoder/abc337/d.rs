#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, k) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut cnt_x = vec![];
    let mut cnt_d = vec![];
    for r in 0..h {
        cnt_x.push(mapv(&arr[r], |&c| if c == 'x' { 1 } else { 0 }));
        cnt_d.push(mapv(&arr[r], |&c| if c == '.' { 1 } else { 0 }));
    }

    let pref_x = build_2d(&cnt_x);
    let pref_d = build_2d(&cnt_d);

    let inf = 1_000_000_000;
    let mut ans = inf;

    // vertical
    for r in (0..).take_while(|r| r + k <= h) {
        for c in 0..w {
            if query_2d(&pref_x, r, c, r + k - 1, c) == 0 {
                ans = ans.min(query_2d(&pref_d, r, c, r + k - 1, c));
            }
        }
    }

    // horizontal
    for c in (0..).take_while(|c| c + k <= w) {
        for r in 0..h {
            if query_2d(&pref_x, r, c, r, c + k - 1) == 0 {
                ans = ans.min(query_2d(&pref_d, r, c, r, c + k - 1));
            }
        }
    }

    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn build_2d<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Default + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    assert!(arr.len() >= 1 && arr[0].len() >= 1);
    let (n, m) = (arr.len(), arr[0].len());
    let mut pref = vec![vec![T::default(); m]; n];
    for r in 0..n {
        for c in 0..m {
            pref[r][c] = arr[r][c];
            if r >= 1 {
                pref[r][c] = pref[r][c] + pref[r - 1][c];
            }
            if c >= 1 {
                pref[r][c] = pref[r][c] + pref[r][c - 1];
            }
            if r >= 1 && c >= 1 {
                pref[r][c] = pref[r][c] - pref[r - 1][c - 1];
            }
        }
    }
    pref
}

// arr[r1..=r2, c1..=c2]
fn query_2d<T>(pref: &Vec<Vec<T>>, r1: usize, c1: usize, r2: usize, c2: usize) -> T
where
    T: Default + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    let mut res = pref[r2][c2];
    if r1 >= 1 {
        res = res - pref[r1 - 1][c2];
    }
    if c1 >= 1 {
        res = res - pref[r2][c1 - 1];
    }
    if r1 >= 1 && c1 >= 1 {
        res = res + pref[r1 - 1][c1 - 1];
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
