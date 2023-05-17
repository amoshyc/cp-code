#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (n, t) = (inp[0] as usize, inp[1]);
    let arr = readv::<u64>();

    let mut pref = vec![0; n];
    pref[0] = arr[0];
    for i in 1..n {
        pref[i] = pref[i - 1] + arr[i];
    }

    let r = t % pref[n - 1];
    for i in 0..n {
        let p = if i == 0 { 0 } else { pref[i - 1] };
        if (p..pref[i]).contains(&r) {
            println!("{} {}", i + 1, r - p);
            break;
        }
    }
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
