#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let ops = readv::<u64>();

    let mut ans = vec![];
    for op in ops {
        ans.push(op);
        while ans.len() > 1 {
            let x = ans[ans.len() - 1];
            let y = ans[ans.len() - 2];
            if x == y {
                ans.pop();
                ans.pop();
                ans.push(x + 1);
            } else {
                break;
            }
        }
    }

    println!("{}", ans.len());
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
