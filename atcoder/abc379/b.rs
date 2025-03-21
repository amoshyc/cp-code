#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let mut s = reads();

    let mut ans = 0;
    for i in ((k - 1)..n) {
        if s[(i + 1 - k)..=i].iter().all(|c| *c == 'O') {
            ans += 1;
            for j in (i - k + 1)..=i {
                s[j] = 'x';
            }
        }
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
