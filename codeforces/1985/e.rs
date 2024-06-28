#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<i64>();
        let (x, y, z, k) = (inp[0], inp[1], inp[2], inp[3]);

        let mut ans = 0;
        for a in 1..=x {
            for b in 1..=y {
                if k % (a * b) != 0 {
                    continue;
                }

                let c = k / a / b;
                let cnt = (x - a + 1) * (y - b + 1) * (z - c + 1);
                ans = ans.max(cnt);
            }
        }

        println!("{}", ans);
    }
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
