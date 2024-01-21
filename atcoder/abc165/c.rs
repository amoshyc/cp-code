#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);
    let mut rel = vec![];
    for _ in 0..q {
        let inp = readv::<i64>();
        let (a, b) = (inp[0] as usize - 1, inp[1] as usize - 1);
        let (c, d) = (inp[2] as usize, inp[3]);
        rel.push((a, b, c, d));
    }

    let mut ans = 0;
    let mut arr = vec![0; n];
    dfs(0, &mut arr, &mut ans, n, m, &rel);

    println!("{}", ans);
}

fn dfs(
    i: usize,
    arr: &mut Vec<usize>,
    ans: &mut i64,
    n: usize,
    m: usize,
    rel: &Vec<(usize, usize, usize, i64)>,
) {
    if i == n {
        let mut s = 0;
        for &(a, b, c, d) in rel.iter() {
            if arr[b] == arr[a] + c {
                s += d;
            }
        }
        *ans = (*ans).max(s);
    } else {
        let last = if i == 0 { 1 } else { arr[i - 1] };
        for j in last..=m {
            arr[i] = j;
            dfs(i + 1, arr, ans, n, m, rel);
            arr[i] = 0;
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
