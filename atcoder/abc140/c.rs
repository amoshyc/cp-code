#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr_b = readv::<usize>();
    arr_b.insert(0, 1_000_000);

    let mut arr_a = vec![0; n];
    for i in 0..n {
        let mut ub = arr_b[i];
        if i + 1 < n {
            ub = ub.min(arr_b[i + 1]);
        }
        arr_a[i] = ub;
    }
    println!("{}", arr_a.iter().sum::<usize>());
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
