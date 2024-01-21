#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();
    let q = read::<usize>();

    let mut cmds = vec![];
    for i in 0..n {
        cmds.push((1, i, s[i]));
    }

    for i in 0..q {
        let inp = readv::<String>();
        let op = inp[0].parse::<usize>().unwrap();
        let p = inp[1].parse::<usize>().unwrap();
        let x = inp[2].chars().next().unwrap();
        if op == 1 {
            cmds.push((1, p - 1, x));
        } else {
            cmds.push((op, 0, ' '));
        }
    }

    let mut ans = vec![' '; n];
    let mut last = 'n';
    for &(op, p, x) in cmds.iter().rev() {
        if op == 2 && last == 'n' {
            last = 'l';
        }
        if op == 3 && last == 'n' {
            last = 'u';
        }

        if op == 1 && ans[p] == ' ' {
            if last == 'n' {
                ans[p] = x;
            } else if last == 'l' {
                ans[p] = x.to_ascii_lowercase();
            } else {
                ans[p] = x.to_ascii_uppercase();
            }
        }
    }

    println!("{}", join(&ans, ""));
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
