#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut conds = vec![];
    for _ in 0..m {
        let inp = readv::<usize>();
        conds.push((inp[0] - 1, inp[1] - 1));
    }

    let k = read::<usize>();
    let mut arr = vec![];
    for _ in 0..k {
        let inp = readv::<usize>();
        arr.push((inp[0] - 1, inp[1] - 1));
    }

    let mut ans = 0;
    for mask in 0..(1 << k) {
        let mut dish = vec![0; n];
        for i in 0..k {
            if (mask >> i) & 1 == 0 {
                dish[arr[i].0] = 1;
            } else {
                dish[arr[i].1] = 1;
            }
        }

        let mut cnt = 0;
        for &(a, b) in conds.iter() {
            if dish[a] == 1 && dish[b] == 1 {
                cnt += 1;
            }
        }

        ans = ans.max(cnt);
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
