#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut s = reads();
    let mut f = vec![0; n];

    for i in (0..).take_while(|i| i + 2 < n) {
        if s[i..=i + 2] == ['A', 'B', 'C'] {
            f[i] = 1;
        }
    }

    let mut cnt = f.iter().sum::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<String>();
        let x = inp[0].parse::<usize>().unwrap() - 1;
        let c = inp[1].chars().next().unwrap();

        s[x] = c;
        for i in x.checked_sub(2).unwrap_or(0)..=x {
            if f[i] == 1 {
                cnt -= 1;
                f[i] = 0;
            }
            if i + 2 < n && s[i..=i + 2] == ['A', 'B', 'C'] {
                f[i] = 1;
                cnt += 1;
            }
        }

        ans.push(cnt);
    }

    println!("{}", join(&ans, "\n"));
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
