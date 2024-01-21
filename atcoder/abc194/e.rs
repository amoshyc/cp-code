#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let max = *arr.iter().max().unwrap();

    let mut inv = std::collections::BTreeSet::new();
    for i in 0..=(max + 1) {
        inv.insert(i);
    }

    let mut win = vec![0; max + 1];
    for i in 0..m {
        win[arr[i]] += 1;
        if win[arr[i]] == 1 {
            inv.remove(&arr[i]);
        }
    }

    let mut ans = *inv.iter().next().unwrap();

    for r in m..n {
        let l = r - m;
        win[arr[l]] -= 1;
        if win[arr[l]] == 0 {
            inv.insert(arr[l]);
        }

        win[arr[r]] += 1;
        if win[arr[r]] == 1 {
            inv.remove(&arr[r]);
        }

        let mex = *inv.iter().next().unwrap();
        ans = ans.min(mex);
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
