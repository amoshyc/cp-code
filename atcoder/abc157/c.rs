#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![10; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (p, x) = (inp[0] - 1, inp[1]);
        if arr[p] == 10 || arr[p] == x {
            arr[p] = x;
        } else {
            println!("-1");
            return;
        }
    }

    if n == 1 {
        println!("{}", arr[0] % 10);
        return;
    }

    if arr[0] == 0 {
        println!("-1");
        return;
    }

    for i in 0..n {
        if arr[i] == 10 {
            if i == 0 {
                arr[i] = 1;
            } else {
                arr[i] = 0;
            }
        }
    }

    println!("{}", join(&arr, ""));
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
    read::<String>().chars().collect::<_>()
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
