#![allow(unused)]

fn main() {
    // Greedily use the one with minimum a - b

    let inp = readv::<i64>();
    let (mut n, m) = (inp[0], inp[1] as usize);

    let mut arr = vec![];
    for i in 0..m {
        let inp = readv::<i64>();
        arr.push((inp[0], inp[1]));
    }
    arr.sort_by_key(|&(a, b)| (a - b, a, b));

    let mut ans = 0;
    for (a, b) in arr {
        if n < a {
            continue;
        }

        let cnt = (n - a) / (a - b) + 1;
        n -= cnt * (a - b);
        ans += cnt;
    }

    println!("{}", ans);
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
