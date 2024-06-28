#![allow(unused)]

fn solve() {
    let n = read::<usize>();
    let mut m = vec![];
    let mut f = vec![];
    for i in 0..n {
        let inp = readv::<String>();
        m.push(inp[0].parse::<u64>().unwrap());
        f.push(inp[1].clone());
    }

    let mut map = std::collections::HashMap::new();
    for i in 0..n {
        if !map.contains_key(&f[i]) || map[&f[i]] > m[i] {
            map.insert(f[i].clone(), m[i]);
        }
    }

    let mut ans = !0;
    for i in 0..n {
        if f[i] == "11" {
            ans = ans.min(m[i]);
        }
        if f[i] == "10" && map.contains_key("01") {
            ans = ans.min(m[i] + map["01"]);
        }
        if f[i] == "01" && map.contains_key("10") {
            ans = ans.min(m[i] + map["10"]);
        }
    }

    if ans == !0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
