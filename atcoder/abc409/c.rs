#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, l) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut cnt = vec![0i64; l];
    let mut pos = 0;
    cnt[0] += 1;
    for x in arr {
        pos += x;
        cnt[pos % l] += 1;
    }

    let d = l / 3;
    let mut ans = 0;
    for j in 0..l {
        let i = (j + l - d) % l;
        let k = (j + d) % l;
        ans += cnt[i] * cnt[j] * cnt[k];
    }

    println!("{}", ans / 3);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
