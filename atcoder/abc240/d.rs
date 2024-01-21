#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut ans = vec![];
    let mut stk = vec![];
    let mut cnt = 0;
    for &a in arr.iter() {
        if stk.len() == 0 {
            stk.push((a, 1));
            cnt += 1;
        } else if stk.last().unwrap().0 == a {
            let (_, c) = stk.pop().unwrap();
            cnt -= c;
            if c + 1 != a {
                stk.push((a, c + 1));
                cnt += c + 1;
            }
        } else {
            stk.push((a, 1));
            cnt += 1;
        }
        ans.push(cnt);
    }

    println!("{}", join(&ans, "\n"));
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
