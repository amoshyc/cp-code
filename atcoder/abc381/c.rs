#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut tokens = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && s[i] == s[j] {
            j += 1;
        }
        tokens.push((s[i], j - i));
        i = j;
    }

    let mut ans = 1;
    for i in 0..tokens.len() {
        if tokens[i].0 == '/' {
            if i >= 1 && i + 1 < tokens.len() {
                if tokens[i - 1].0 == '1' && tokens[i + 1].0 == '2' {
                    let l = tokens[i - 1].1.min(tokens[i + 1].1);
                    ans = ans.max(2 * l + 1);
                }
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
