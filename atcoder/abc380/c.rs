#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let s = reads();

    let mut tokens = vec![];
    let mut types = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && s[j] == s[i] {
            j += 1;
        }
        tokens.push(join(&s[i..j], ""));
        types.push(s[i]);
        i = j;
    }

    let mut cnt = 0;
    let mut a = 0;
    let mut b = 0;
    for i in 0..tokens.len() {
        if types[i] == '1' {
            cnt += 1;
            if cnt == k - 1 {
                a = i;
            }
            if cnt == k {
                b = i;
            }
        }
    }

    let x = tokens.remove(b);
    tokens.insert(a + 1, x);
    println!("{}", join(&tokens, ""));
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
