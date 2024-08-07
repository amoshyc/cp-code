#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut l = vec![0; n];
    let mut r = vec![0; n];
    let mut target_y = 0;
    for i in 0..n {
        let inp = readv::<i64>();
        l[i] = inp[0];
        r[i] = inp[1];
    }

    let sum_l = l.iter().sum::<i64>();
    let sum_r = r.iter().sum::<i64>();
    if target_y < sum_l || target_y > sum_r {
        println!("No");
        return;
    }

    let mut y = l.clone();
    target_y -= sum_l;
    for i in 0..n {
        if target_y > 0 {
            let x = target_y.min(r[i] - l[i]);
            y[i] += x;
            target_y -= x;
        }
    }

    println!("Yes");
    println!("{}", join(&y, " "));
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
