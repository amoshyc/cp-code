#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<i64>();

        let mut ans = 0;
        let mut max = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += arr[i];
            max = max.max(arr[i]);
            if max == sum - max {
                ans += 1;
            }
        }

        println!("{}", ans);
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
