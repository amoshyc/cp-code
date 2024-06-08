#![allow(unused)]

// [Problem]
// In a two-dimensional coordinate plane, there are N distinct points P1, P2, ..., PN, with the coordinates of point Pi being (xi, yi).
// Process the following Q queries in order.
// For the i-th query (1 ≤ i ≤ Q), an integer qi is given.
// Output the maximum Manhattan distance between point Pqi and the N points.
// In other words, if the Manhattan distance between point Ps and point Pt is dist(Ps, Pt), output the value of max(dist(Pqi, P1), ..., dist(Pqi, PN)).

// [Solution]
// https://amoshyc.github.io/cp-codebook/techniques/l1.html#2d-points
// The solution for the i-th query is
//    max(
//      u[i] - min(u[j] for j in 0..n),
//      v[i] - min(v[j] for j in 0..n),
//      max(v[j] for j in 0..n) - v[i],
//      max(u[j] for j in 0..n) - u[i],
//    )

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut u = vec![];
    let mut v = vec![];
    for _ in 0..n {
        let xy = readv::<i64>();
        let (x, y) = (xy[0], xy[1]);
        u.push(x + y);
        v.push(x - y);
    }

    let max_u = u.iter().max().unwrap();
    let min_u = u.iter().min().unwrap();
    let max_v = v.iter().max().unwrap();
    let min_v = v.iter().min().unwrap(); 

    let mut ans = vec![];
    for _ in 0..q {
        let i = read::<usize>() - 1;
        let rhs = [u[i] - min_u, v[i] - min_v, max_v - v[i], max_u - u[i]];
        ans.push(*rhs.iter().max().unwrap());
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
