#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0] as usize, inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        arr.push((inp[0], inp[1]));
    }
    arr.sort();
    arr.push((10i64.pow(18), 0));

    let mut pos = 0;
    let mut val = k;
    for (a, b) in arr {
        // from pos to a
        let need = a - pos;
        if val >= need {
            pos = a;
            val -= need;
            val += b;
        } else {
            pos += val;
            val = 0;
            break;
        }
    }

    println!("{}", pos);
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
