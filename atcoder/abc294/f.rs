#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    for _ in 0..n {
        let inp = readv::<f64>();
        arr1.push((inp[0], inp[1]));
    }
    for _ in 0..m {
        let inp = readv::<f64>();
        arr2.push((inp[0], inp[1]));
    }

    // k-th largest <-> k - 1 one bigger
    // ok(m) = are there k - 1 or less one with concentration > m
    // 0 0 0 1 1 1
    let ok = |m: f64| {
        // sum_(i, j) [ (a[i] + c[j]) / (a[i] + b[i] + c[j] + d[j]) > m ]
        // sum_(i, j) [ a[i] + c[j] > m * a[i] + m * b[i] + m * c[j] + m * d[j] ]
        // sum_(i, j) [ (m - 1) * a[i] + m * b[i] < (1 - m) * c[j] - m * d[j] ]
        let mut rhs = arr2
            .iter()
            .map(|&(c, d)| (1.0 - m) * c - m * d)
            .collect::<Vec<_>>();

        rhs.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let mut cnt = 0;
        for &(a, b) in arr1.iter() {
            let q = (m - 1.0) * a + m * b;
            let j = partition_point(&rhs, |&x| x <= q);
            cnt += arr2.len() - j;
        }
        cnt <= k - 1
    };

    let mut lb = 0.0 as f64;
    let mut ub = 1.0 as f64;
    for _ in 0..50 {
        let m = (lb + ub) / 2.0;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }

    println!("{:.10}", ub * 100.0);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
