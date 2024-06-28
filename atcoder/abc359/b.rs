#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut pos = vec![vec![]; n];
    let arr = readv::<usize>();
    let arr = mapv(&arr, |&x| x - 1);
    for i in 0..(2 * n) {
        pos[arr[i]].push(i);
    }

    let mut ans = 0;
    for i in 0..n {
        let (a, b) = (pos[i][0], pos[i][1]);
        if a + 2 == b {
            ans += 1;
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
