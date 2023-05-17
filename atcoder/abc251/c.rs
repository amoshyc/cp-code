#![allow(unused)]

fn main() {
    let n = read::<usize>();
    
    let mut s = std::collections::HashSet::new();
    let mut best_idx = 0;
    let mut best_val = 0;
    for i in 0..n {
        let inp = readv::<String>();
        let sub = inp[0].clone();
        let val = inp[1].parse::<usize>().unwrap();

        if s.contains(&sub) {
            continue;
        } else {
            s.insert(sub);
            if val > best_val {
                best_idx = i;
                best_val = val;
            }
        }
    }

    println!("{}", best_idx + 1);
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
