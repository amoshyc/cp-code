#![allow(unused)]

fn main() {
    let s = reads();
    let mut ans = 0;
    for (x, c, _) in rle(&s) {
        if x == '0' {
            ans += (c + 1) / 2;
        } else {
            ans += c;
        }
    }
    println!("{}", ans);
}

fn rle<T: Copy + PartialEq>(arr: &Vec<T>) -> Vec<(T, usize, usize)> {
    let n = arr.len();
    let mut res = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && arr[j] == arr[i] {
            j += 1;
        }
        res.push((arr[i], j - i, i));
        i = j;
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
