#![allow(unused)]

fn main() {
    let mut arr = vec![];
    for _ in 0..3 {
        arr.push(readv::<usize>());
    }

    let n = read::<usize>();
    let mut f = vec![false; 101];
    for _ in 0..n {
        let x = read::<usize>();
        f[x] = true;
    }

    let mut ans = false;
    ans |= (0..3).all(|c| f[arr[0][c]]);
    ans |= (0..3).all(|c| f[arr[1][c]]);
    ans |= (0..3).all(|c| f[arr[2][c]]);
    ans |= (0..3).all(|r| f[arr[r][0]]);
    ans |= (0..3).all(|r| f[arr[r][1]]);
    ans |= (0..3).all(|r| f[arr[r][2]]);
    ans |= (0..3).all(|i| f[arr[i][i]]);
    ans |= (0..3).all(|i| f[arr[i][2 - i]]);
    if ans {
        println!("Yes");
    } else {
        println!("No");
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
    read::<String>().chars().collect::<_>()
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
