#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![String::new(); tc];
    for tid in 0..tc {
        let n = read::<usize>();
        let mut s = reads();

        if n == 1 {
            ans[tid] = join(&s, "");
            continue;
        }

        if let Some(l) = (0..(n - 1)).find(|&i| s[i] > s[i + 1]) {
            let r = ((l + 1)..n).find(|&i| s[i] > s[l]).unwrap_or(n);
            s[l..r].rotate_left(1);
        }

        ans[tid] = join(&s, "");
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
