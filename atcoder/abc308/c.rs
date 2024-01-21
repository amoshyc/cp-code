#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    let mut ans: Vec<usize> = (0..n).collect();
    ans.sort_by(|&i, &j| {       
        let fi = a[i] * (a[j] + b[j]);
        let fj = a[j] * (a[i] + b[i]);
        if fi == fj {
            i.cmp(&j)
        } else {
            fj.cmp(&fi)
        }
    });

    let mut ans: Vec<String> = ans.iter().map(|&x| (x + 1).to_string()).collect();
    println!("{}", ans.join(" "));
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
