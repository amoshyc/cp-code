#![allow(unused)]


fn main() {
    let n = read::<u64>();
    let arr = readv::<usize>();
    let max = *arr.iter().max().unwrap();

    let mut cnt = vec![0 as u64; max + 1];
    for &x in arr.iter() {
        cnt[x] += 1;
    }

    // answer = n(universe) - n(A[i] = A[j] or A[j] = A[k])
    //        = n(universe) - (n(A[i] = A[j] ≠ A[k]) + n(A[i] = A[j] = A[k]))
    let mut ans = comb(n as u64, 3);
    for x in 0..=max {
        if cnt[x] >= 2 {
            ans = ans - comb(cnt[x], 2) * (n - cnt[x]);
        }
        if cnt[x] >= 3 {
            ans = ans - comb(cnt[x], 3);
        }
    }

    println!("{}", ans);
}

fn comb(a: u64, b: u64) -> u64 {
    let mut res = 1;
    for i in 1..=b {
        res = res * (a + 1 - i);
        res = res / i;
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
