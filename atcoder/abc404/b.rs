#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut s = vec![];
    for _ in 0..n {
        s.push(reads());
    }

    let mut t = vec![];
    for _ in 0..n {
        t.push(reads());
    }

    let mut ans = usize::MAX;
    for i in 0..4 {
        let cnt = cartesian(0..n, 0..n)
            .map(|(r, c)| if s[r][c] != t[r][c] { 1 } else { 0 })
            .sum::<usize>();
        ans = ans.min(i + cnt);
        s = rotate_cw(&s);
    }

    println!("{ans}");
}

// clockwise
//  123        41
//  456   ->   52
//             63
// (2x3)      (3x2)
fn rotate_cw<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![arr[0][0].clone(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[c][n - 1 - r] = arr[r][c].clone();
        }
    }
    res
}

// counter-clockwise
//  123        36
//  456   ->   25
//             14
// (2x3)      (3x2)
fn rotate_ccw<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![arr[0][0].clone(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[m - 1 - c][r] = arr[r][c].clone();
        }
    }
    res
}

fn cartesian<T, R1, R2>(r1: R1, r2: R2) -> impl Iterator<Item = (T, T)>
where
    T: Clone,
    R1: std::ops::RangeBounds<T> + Iterator<Item = T> + Clone,
    R2: std::ops::RangeBounds<T> + Iterator<Item = T> + Clone,
{
    r1.flat_map(move |x| r2.clone().map(move |y| (x.clone(), y)))
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
