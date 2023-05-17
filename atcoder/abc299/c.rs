#![allow(unused)]

fn f(s: &Vec<char>) -> usize {
    // -ooo
    let n = s.len();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if s[i] != '-' {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < n && s[j] == 'o' {
            j += 1;
        }
        ans = ans.max(j - i - 1);
        i = j;
    }
    ans
}

fn main() {
    let n = read::<usize>();
    let mut s = reads();

    let mut ans = 0;
    ans = ans.max(f(&s));
    s.reverse();
    ans = ans.max(f(&s));

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
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
