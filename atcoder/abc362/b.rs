#![allow(unused)]

fn main() {
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..3 {
        let inp = readv::<i64>();
        x.push(inp[0]);
        y.push(inp[1]);
    }

    let d2 = |i: usize, j: usize| (x[i] - x[j]).pow(2) + (y[i] - y[j]).pow(2);

    let mut idx: Vec<usize> = (0..3).collect();
    loop {
        if d2(idx[0], idx[1]) + d2(idx[1], idx[2]) == d2(idx[0], idx[2]) {
            println!("Yes");
            return;
        }

        if next_permutation(&mut idx).is_none() {
            break;
        }
    }

    println!("No");
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
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
