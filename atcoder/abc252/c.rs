#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    for _ in 0..n {
        s.push(reads());
    }

    let mut pos = vec![vec![]; 10];
    for i in 0..n {
        for j in 0..10 {
            pos[s[i][j].to_digit(10).unwrap() as usize].push(j);
        }
    }

    let mut ans = 1_000_000_000;

    for i in 0..10 {
        pos[i].sort();

        let mut used = std::collections::HashSet::new();
        for j in 0..n {
            let mut x = pos[i][j];
            while used.contains(&x) {
                x = x + 10;
            }
            used.insert(x);
        }

        ans = ans.min(*used.iter().max().unwrap());
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
