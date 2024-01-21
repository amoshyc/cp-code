#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, m, d) = (inp[0] as usize, inp[1] as usize, inp[2] as i64);
    let mut a = readv::<i64>();
    let mut b = readv::<i64>();
    
    b.sort();
    let mut ans = -1;
    for i in 0..n {
        let j = partition_point(&b, |&x| x <= a[i] + d);
        if j == 0 {
            continue;
        }
        if (b[j - 1] - a[i]).abs() > d {
            continue;
        }
        ans = ans.max(a[i] + b[j - 1]);
    }
    println!("{}", ans);
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
