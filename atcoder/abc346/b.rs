#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (w, b) = (inp[0], inp[1]);

    let mut s = vec![];
    for _ in 0..100 {
        s.extend("wbwbwwbwbwbw".chars().collect::<Vec<char>>());
    }

    let n = s.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let cnt_w = s[i..j].iter().filter(|&c| *c == 'w').count();
            let cnt_b = s[i..j].iter().filter(|&c| *c == 'b').count();
            if (w, b) == (cnt_w, cnt_b) {
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
