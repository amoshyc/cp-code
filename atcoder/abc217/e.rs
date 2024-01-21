#![allow(unused)]

fn main() {
    let mut deq = std::collections::VecDeque::new();
    let mut set = std::collections::BTreeSet::new();

    let mut ans = vec![];
    let q = read::<usize>();
    for i in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            let x = inp[1];
            deq.push_back((x, i));
        } else if inp[0] == 2 {
            if set.is_empty() {
                let (x, i) = deq.pop_front().unwrap();
                ans.push(x);
            } else {
                let (x, i) = *set.iter().next().unwrap();
                set.remove(&(x, i));
                ans.push(x);
            }
        } else {
            while let Some(x) = deq.pop_front() {
                set.insert(x);
            }
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
