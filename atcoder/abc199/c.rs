#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = reads();

    let mut f = false;
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            let mut a = inp[1] - 1;
            let mut b = inp[2] - 1;
            if f {
                if a >= n {
                    a -= n;
                } else {
                    a += n;
                }
                if b >= n {
                    b -= n;
                } else {
                    b += n;
                }
            }
            s.swap(a, b);
        } else {
            f = !f;
        }
    }

    if f {
        s.rotate_left(n);
    }
    println!("{}", join(&s, ""));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
