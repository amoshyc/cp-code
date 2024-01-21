#![allow(unused)]

fn main() {
    let q = read::<usize>();

    let mut set = std::collections::BTreeMap::new();
    let mut ans = vec![];

    for _ in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            let x = inp[1];
            *set.entry(x).or_insert(0) += 1;
        } else if inp[0] == 2 {
            let (x, c) = (inp[1], inp[2]);
            if set.contains_key(&x) {
                let n = set[&x];
                let m = c.min(n);
                set.remove(&x);
                if n - m > 0 {
                    set.insert(x, n - m);
                }
            }
        } else {
            let r = set.range(..);
            let (max, _) = r.clone().last().unwrap();
            let (min, _) = r.clone().next().unwrap();
            ans.push(max - min);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
