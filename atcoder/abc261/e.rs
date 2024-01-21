#![allow(unused)]

fn main() {
    let inp = readv::<u32>();
    let (n, c) = (inp[0] as usize, inp[1]);
    let mut cmds = vec![];
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<u32>();
        cmds.push(inp[0]);
        arr.push(inp[1]);
    }

    let mut ans = vec![0; n];
    for j in 0..30 {
        let mut pref_0 = 0;
        let mut pref_1 = 1;
        for i in 0..n {
            let a = (arr[i] >> j) & 1;
            match cmds[i] {
                1 => {
                    pref_0 = (pref_0 & a);
                    pref_1 = (pref_1 & a);
                }
                2 => {
                    pref_0 = (pref_0 | a);
                    pref_1 = (pref_1 | a);
                }
                _ => {
                    pref_0 = (pref_0 ^ a);
                    pref_1 = (pref_1 ^ a);
                }
            }
            let prev = if i == 0 { c } else { ans[i - 1] };
            let bit = if (prev >> j) & 1 == 1 { pref_1 } else { pref_0 };
            ans[i] |= (bit << j);
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
