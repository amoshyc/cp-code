#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let n = read::<usize>();
        let s = reads();

        let mut i = 0;
        let mut max = 0;
        while i < n {
            let mut j = i;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            max = max.max(j - i);
            i = j;
        }

        println!("{:?}", max + 1);
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
