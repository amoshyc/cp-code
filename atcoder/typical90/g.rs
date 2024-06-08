#![allow(unused)]

// [Problem]
// At the ABC Programming School, there are N classes, and class number i (1≤i≤N) targets students with a rating of Ai​.
//
// Now, Q students are trying to join this school.
// The rating of student number j (1≤j≤Q) is Bj​.
// Each student becomes dissatisfied if they go to a class that doesn't match their level.
// For each student, their dissatisfaction is defined as follows:
//     The absolute difference between the target rating a and their own rating b, that is, ∣a−b∣
//
// For each j=1,2,3,…,Q, please find the minimum dissatisfaction that can be considered for student number j.
// It is allowed to have classes with no students and classes with multiple students.

// [Solution]
// For a B[j], binary search on the A and check the neighboring classes.
// Sample Input 1:
//     A           3200, 4000, 4400, 5000
// b[0] = 3312      ^     ^
// b[1] = 2992      ^
// b[2] = 4229            ^      ^

fn main() {
    let n = read::<usize>();
    let mut a = readv::<i64>();
    a.sort();

    let q = read::<usize>();
    let mut ans = vec![i64::MAX; q];
    for i in 0..q {
        let b = read::<i64>();
        let p = a.partition_point(|r| *r < b);
        if p < n {
            ans[i] = ans[i].min((a[p] - b).abs());
        }
        if p >= 1 {
            ans[i] = ans[i].min((a[p - 1] - b).abs());
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
