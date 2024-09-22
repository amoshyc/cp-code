#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let h = readv::<i64>();

    let mut stk = vec![];
    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        while let Some(j) = stk.last() {
            if h[i] > h[*j] {
                stk.pop();
            } else {
                break;
            }
        }
        stk.push(i);
        if i > 0 {
            ans[i - 1] = stk.len();
        }
    }
    println!("{}", join(&ans, " "));

    // Failed at h=`8 4 9 7 7`.
    // let inf = 10_000_000;
    // let mut h = mapv(&h, |&x| (inf - 1 - x));
    // h.reverse();
    // let mut lis = longest_increasing_subsequence(&h, false, inf);
    // lis.reverse();
    // lis.push(0);
    // println!("{}", join(&lis[1..], " "));
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
