#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut set = std::collections::HashSet::new();
    for _ in 0..n {
        let mut s = reads();
        let mut t = s.clone();
        if t[0] == '!' {
            t.remove(0);
        } else {
            t.insert(0, '!');
        }

        if set.contains(&t) {
            if s[0] == '!' {
                s.remove(0);
            }
            println!("{}", join(&s, ""));
            return;
        } else {
            set.insert(s);
        }
    }

    println!("satisfiable");
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
