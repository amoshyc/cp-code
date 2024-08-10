#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![vec![vec![0; n]; n]; n];
    for x in 0..n {
        for y in 0..n {
            arr[x][y] = readv::<i64>();
        }
    }

    let mut pref = vec![];
    for x in 0..n {
        pref.push(build_2d(&arr[x]));
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (x_min, x_max) = (ask[0] - 1, ask[1] - 1);
        let (y_min, y_max) = (ask[2] - 1, ask[3] - 1);
        let (z_min, z_max) = (ask[4] - 1, ask[5] - 1);
        let mut res = 0;
        for x in x_min..=x_max {
            res += query_2d(&pref[x], y_min, z_min, y_max, z_max);
        }
        ans.push(res);
    }

    println!("{}", join(&ans, "\n"));
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
