#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let arr = mapv(&arr, |&x| x - 1);

    let mut groups = vec![vec![]; n];
    for i in 0..n {
        groups[arr[i]].push(i);
    }

    let mut ans = 0;
    for x in 0..n {
        for (i, w) in groups[x].windows(2).enumerate() {
            ans += (w[1] - w[0] - 1) * (i + 1) * (groups[x].len() - i - 1);
        }
    }

    println!("{}", ans);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
