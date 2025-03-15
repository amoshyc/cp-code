#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut jobs = vec![];
    for _ in 0..n {
        let ab = readv::<usize>();
        jobs.push((ab[0], ab[1]));
    }

    jobs.sort_by_key(|(a, b)| *b);
    let mut t = 0;
    for (a, b) in jobs {
        t += a;
        if t > b {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
