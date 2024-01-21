#![allow(unused)]

fn main() {
    let q = read::<usize>();

    let mut que = std::collections::VecDeque::new();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<i64>();
        if inp[0] == 1 {
            let (x, c) = (inp[1], inp[2]);
            que.push_back((c, x));
        } else {
            let mut c = inp[1];
            let mut val = 0;
            while c > 0 {
                let (k, x) = que.pop_front().unwrap();
                if k >= c {
                    val += c * x;
                    que.push_front((k - c, x));
                    c = 0;
                } else {
                    val += k * x;
                    c -= k;
                }
            }
            ans.push(val);
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
