#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr_a = readv::<usize>();
    let arr_b = readv::<usize>();
    let arr_c = readv::<usize>();

    let arr_a = mapv(&arr_a, |&x| x - 1);
    let mut sum = 0;
    for i in 0..n {
        sum += arr_b[arr_a[i]];
        if i + 1 < n && arr_a[i + 1] == arr_a[i] + 1 {
            sum += arr_c[arr_a[i]];
        }
    }
    println!("{sum}");
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
