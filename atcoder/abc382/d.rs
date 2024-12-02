#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut arr = vec![0; n];
    let mut res = vec![];
    dfs(0, m, &mut arr, &mut res);

    println!("{}", res.len());
    println!("{}", join(&res, "\n"));
}

fn dfs(i: usize, m: usize, arr: &mut Vec<usize>, res: &mut Vec<String>) {
    let n = arr.len();
    if i == n {
        res.push(join(&arr, " "));
        return;
    }

    // A[i] - A[i - 1] >= 10
    let lb = if i == 0 { 1 } else { arr[i - 1] + 10 };
    let ub = m.saturating_sub((n - i - 1) * 10);
    for v in lb..=ub {
        arr[i] = v;
        dfs(i + 1, m, arr, res);
        arr[i] = 0;
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
