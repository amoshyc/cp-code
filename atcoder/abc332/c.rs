#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let s = reads();

    let check = |mid: usize| {
        let mut plain = m;
        let mut logo = mid;
        let mut ok = true;

        for i in 0..n {
            if s[i] == '0' {
                plain = m;
                logo = mid;
            } else if s[i] == '1' {
                if plain >= 1 {
                    plain -= 1;
                } else if logo >= 1 {
                    logo -= 1;
                } else {
                    ok = false;
                }
            } else {
                if logo == 0 {
                    ok = false;
                } else {
                    logo -= 1;
                }
            }
        }

        ok
    };

    // 0 0 0 1 1 1
    let mut lb = 0;
    let mut ub = n + 1;
    if check(lb) {
        println!("{}", lb);
        return;
    }

    while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        if check(mid) {
            ub = mid;
        } else {
            lb = mid;
        }
    }

    println!("{}", ub);
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
