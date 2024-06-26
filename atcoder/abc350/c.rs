#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let mut arr = mapv(&arr, |&x| x - 1);

    let mut pos = vec![0; n];
    for (i, x) in arr.iter().enumerate() {
        pos[*x] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        let x = arr[i];
        if x == i {
            continue;
        }
        let pi = pos[i];
        let px = pos[x];
        ans.push(format!("{} {}", pi.min(px) + 1, pi.max(px) + 1));
        arr.swap(pi, px);
        pos[x] = pi;
        pos[i] = i;
    }

    println!("{}", ans.len());
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
    read::<String>().chars().collect::<_>()
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
