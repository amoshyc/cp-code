#![allow(unused)]

// [Problem]
// At ABC University, there are N freshmen enrolled.
// There are two classes, and the class of student with student ID i is class Ci.
// Today, the final exam scores of the students with student ID i were Pi.
// You are given Q queries in the following format. For each j=1,2,...,Q please answer:
//    * The total final exam scores of students in class 1 from student ID Lj to Rj.
//    * The total final exam scores of students in class 2 from student ID Lj to Rj.
//    * Calculate these two values separately.

// [Solution]
// Prefix Sum

fn main() {
    let n = read::<usize>();
    let mut scores = vec![vec![0; n]; 2];
    for i in 0..n {
        let inp = readv::<usize>();
        let (c, s) = (inp[0], inp[1] as i64);
        scores[c - 1][i] = s;
    }

    let prefs = vec![build(&scores[0]), build(&scores[1])];

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (l, r) = (inp[0] - 1, inp[1] - 1);
        ans.push(format!(
            "{} {}",
            query(&prefs[0], l, r + 1),
            query(&prefs[1], l, r + 1)
        ));
    }
    println!("{}", join(&ans, "\n"));
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
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
    read::<String>().chars().collect::<_>()
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
