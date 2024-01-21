#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();
    let w = readv::<usize>();

    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by_key(|&i| w[i]);
    let s = indices
        .iter()
        .map(|&i| (s[i] == '1') as usize)
        .collect::<Vec<_>>();
    let w = indices.iter().map(|&i| w[i]).collect::<Vec<_>>();

    let pref = build(&s);

    let mut ans = std::cmp::max(query(&pref, 0, n), n - query(&pref, 0, n));
    for i in 0..n {
        let j = partition_point(&w, |&x| x < w[i]);
        let left_0 = j - query(&pref, 0, j);
        let right_1 = query(&pref, j, n);
        ans = ans.max(left_0 + right_1);
    }

    println!("{}", ans);
}

fn build(arr: &Vec<usize>) -> Vec<usize> {
    let mut pref = vec![arr[0]];
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

fn query(pref: &Vec<usize>, l: usize, r: usize) -> usize {
    if l == r {
        0
    } else {
        let mut val = pref[r - 1];
        if l > 0 {
            val -= pref[l - 1];
        }
        val
    }
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
