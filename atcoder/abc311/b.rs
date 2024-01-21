#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, d) = (inp[0], inp[1]);

    let mut arr = vec!['o'; d];
    for _ in 0..n {
        let s = reads();
        for i in 0..d {
            if s[i] == 'x' {
                arr[i] = 'x';
            }
        }
    }

    let mut ans = 0;
    let mut i = 0;
    while i < d {
        if arr[i] == 'x' {
            i = i + 1;
            continue;
        }
        let mut j = i + 1;
        while j < d && arr[j] == 'o' {
            j += 1;
        }
        ans = ans.max(j - i);
        i = j;
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
