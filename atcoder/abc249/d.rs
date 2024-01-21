#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<i64>();
    arr.sort();

    let max = *arr.iter().max().unwrap();

    let mut set = std::collections::HashMap::new();
    for i in 0..n {
        *set.entry(arr[i]).or_insert(0) += 1 as i64;
    }

    let mut cnt = 0;

    for i in 0..=max {
        for j in (0..=max).take_while(|&j| i * j <= max) {
            let ci = set.get(&i).unwrap_or(&0);
            let cj = set.get(&j).unwrap_or(&0);
            let cij = set.get(&(i * j)).unwrap_or(&0);
            cnt += ci * cj * cij;
        }
    }

    println!("{}", cnt);
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
