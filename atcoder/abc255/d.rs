#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut arr = readv::<i64>();
    arr.sort();

    let mut pref = vec![arr[0]];
    for i in 1..n {
        pref.push(pref[i - 1] + arr[i]);
    }

    let sum = |l: usize, r: usize| -> i64 {
        if l == r {
            0
        } else if l == 0 {
            pref[r - 1]
        } else {
            pref[r - 1] - pref[l - 1]
        }
    };

    let mut ans = vec![];
    for _ in 0..q {
        let x = read::<i64>();
        let i = partition_point(&arr, |a| *a <= x);
        let cnt1 = (i as i64) * x - sum(0, i);
        let cnt2 = sum(i, n) - ((n - i) as i64 * x);
        ans.push(cnt1 + cnt2);
    }

    println!("{}", join(&ans, "\n"));
}

// arr.partition_point is added at 1.52.0
// 1 1 1 0 0 0
//       ^
fn partition_point<T, P: FnMut(&T) -> bool>(arr: &[T], mut pred: P) -> usize {
    arr.binary_search_by(|x| {
        if pred(x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
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
