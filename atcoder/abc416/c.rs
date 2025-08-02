#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k, x) = (inp[0], inp[1], inp[2] - 1);
    let mut s = vec![];
    for i in 0..n {
        s.push(reads());
    }

    let mut arr = vec![n; k];
    let mut res = vec![];
    dfs(&mut arr, 0, &mut res, &s);
    res.sort();

    println!("{}", res[x]);
}

fn dfs(arr: &mut Vec<usize>, i: usize, res: &mut Vec<String>, s: &Vec<Vec<char>>) {
    let n = s.len();
    let k = arr.len();

    if i == k {
        let mut t = vec![];
        for i in arr {
            t.extend(s[*i].clone());
        }
        res.push(join(&t, ""));
        return;
    }

    for j in 0..n {
        arr[i] = j;
        dfs(arr, i + 1, res, s);
        arr[i] = n;
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
