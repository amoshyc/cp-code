#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let m = 1_000_000 + 1;
    let mut cnt = vec![0; m];
    for i in 0..n {
        cnt[arr[i]] += 1;
    }

    let mut ans = vec![0; m];
    for gcd in (1..m).rev() {
        let val = (gcd..m).step_by(gcd).map(|m| cnt[m]).sum::<usize>();
        if val >= k {
            for m in (gcd..m).step_by(gcd) {
                ans[m] = ans[m].max(gcd);
            }
        }
    }

    let ans = mapv(&arr, |&x| ans[x]);
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
