#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        let inp = readv::<u64>();
        let (a, b) = (inp[0], inp[1]);

        // 1  12
        // 2  6
        // 3  4

        let g = gcd(a, b);

        // Case 1:
        // 1       x
        // b       b
        if a == 1 && b * b <= 1_000_000_000 {
            ans.push(b * b);
            continue;
        }

        // Case 2:
        // 1       x
        // a       b
        if g == 1 && a * b <= 1_000_000_000 {
            ans.push(a * b);
            continue;
        }

        // Case 3:
        // 1       x
        //         b
        //         a
        let l = lcm(a / g, b / g);
        if g * l == b {
            ans.push(g * (b / g) * (b / g));
        } else {
            ans.push(g * l);
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
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
