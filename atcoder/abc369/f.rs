#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);
    let mut pos = vec![];
    for _ in 0..n {
        let rc = readv::<usize>();
        let (r, c) = (rc[0] - 1, rc[1] - 1);
        pos.push((r, c));
    }
    pos.sort();

    let arr = mapv(&pos, |&(r, c)| c);
    let dp = build_lis_dp(&arr, usize::MAX);
    let mut path = construct_lis(&dp, &pos);
    path.insert(0, (0, 0));
    path.push((h - 1, w - 1));

    let mut ans = vec![];
    for w in path.windows(2) {
        let (r1, c1) = w[0];
        let (r2, c2) = w[1];
        for r in r1..r2 {
            ans.push('D');
        }
        for c in c1..c2 {
            ans.push('R');
        }
    }

    println!("{}", dp.iter().max().unwrap());
    println!("{}", join(&ans, ""));
}

fn build_lis_dp<T>(arr: &Vec<T>, inf: T) -> Vec<usize>
where
    T: Clone + PartialOrd,
{
    // aux[0] is meaningless, so we skip it.
    // weakly: <=, strictly: <
    let n = arr.len();
    let mut aux = vec![inf; n + 1]; // Note the n + 1
    let mut dp = vec![0; n];
    for i in 0..n {
        dp[i] = aux[1..].partition_point(|x| *x <= arr[i]) + 1;
        aux[dp[i]] = arr[i].clone();
    }
    dp
}

fn construct_lis<T: Clone>(dp: &Vec<usize>, arr: &Vec<T>) -> Vec<T> {
    let mut len = *dp.iter().max().unwrap();
    let mut lis = vec![];
    for i in (0..arr.len()).rev() {
        if dp[i] == len {
            lis.push(arr[i].clone());
            len -= 1;
        }
    }
    lis.reverse();
    lis
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
