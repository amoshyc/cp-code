#![allow(unused)]

fn main() {
    let arr = readv::<usize>();
    let n = arr.len();

    for mask in 0..(1usize << n) {
        if mask.count_ones() == 5 {
            let mut x: Vec<usize> = (0..n)
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| arr[i])
                .collect();
            x.sort();

            if x[0] == x[1] && x[1] == x[2] && x[2] != x[3] && x[3] == x[4] {
                println!("Yes");
                return;
            }
            if x[0] == x[1] && x[1] != x[2] && x[2] == x[3] && x[3] == x[4] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
