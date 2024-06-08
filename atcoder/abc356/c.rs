#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let mut tests = vec![];
    for i in 0..m {
        let inp = readv::<String>();
        let r = inp[inp.len() - 1].chars().collect::<Vec<char>>()[0];
        let arr = mapv(&inp[1..inp.len() - 1].to_vec(), |s| {
            s.parse::<usize>().unwrap() - 1
        });
        tests.push((arr, r));
    }

    let mut ans = 0;
    for mask in 0..(1 << n) {
        let mut ok = true;
        for i in 0..m {
            let (arr, r) = tests[i].clone();
            let mut cnt = 0;
            for p in arr.iter() {
                if (mask >> p) & 1 == 1 {
                    cnt += 1;
                }
            }
            if r == 'o' {
                ok &= cnt >= k;
            } else {
                ok &= cnt < k;
            }
        }
        if ok {
            ans += 1;
        }
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
