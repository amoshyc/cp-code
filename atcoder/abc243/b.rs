#![allow(unused)]


fn main() {
    let n = read::<usize>();
    let arr1 = readv::<usize>();
    let arr2 = readv::<usize>();

    let mut cnt0 = 0;
    let mut cnt1 = 0;
    for i in 0..n {
        if arr1[i] == arr2[i] {
            cnt0 += 1;
        }
        for j in 0..n {
            if i != j && arr1[i] == arr2[j] {
                cnt1 += 1;
            }
        }
    }

    println!("{} {}", cnt0, cnt1);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
