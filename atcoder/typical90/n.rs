#![allow(unused)]

// [Problem]
// There are N elementary school students living on AGC Street, and the house of student i (1<=i<=N) is located at position Ai.
// Additionally, there are N elementary schools built along the street, and elementary school j (1<=j<=N) is located at position Bj.
// The elementary school students living on AGC Street have bad personalities, and every pair of them have strained relationships.
// Therefore, it is desired that each student attends a different school.
//
// Moreover, inconvenience is defined as follows:
// * Let Ei be the distance from the house of student i to the school. Inconvenience is the sum of distances, which is E1+E2+...+EN.
// * However, the distance from position u to position v is |u-v|.
//
// Please find the minimum value of inconvenience that can be considered under the condition that each student attends a different school.

// [Solution]
// Greedy: the i-th smallest of A matches to the i-th smallest of B.
// It can be proved by exchange. Given a1, a2, b1, b2 (a1 < b1, a1 < a2, b1 < b2):
// ---------------> position
// a1    a2
//    b1     b2
// If the optimal solution is a1 <-> b2, a2 <-> b1,
// then we always can exchange the matching:  a1 <-> b1, a2 <-> b2
// which has equivalent or smaller inconvenience.
// before = |a1 - b2| + |a2 - b1| = b2 - a1 + a2 - b1
// after = |a1 - b1| + |a2 - b2| = b1 - a1 + b2 - a2
// after - before = 2 * b1 -2 * a1 >= 0

fn main() {
    let n = read::<usize>();
    let mut arr_a = readv::<i64>();
    let mut arr_b = readv::<i64>();
    arr_a.sort();
    arr_b.sort();
    let ans = (0..n).map(|i| (arr_a[i] - arr_b[i]).abs()).sum::<i64>();
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
