#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, l) = (inp[0], inp[1], inp[2]);
    let mut arr1 = readv::<i64>();
    let mut arr2 = readv::<i64>();

    let mut ban = std::collections::HashSet::new();
    for _ in 0..l {
        let inp = readv::<usize>();
        ban.insert((inp[0] - 1, inp[1] - 1));
    }

    let mut indices = argsort(&arr2);
    indices.reverse();

    let mut que = std::collections::BinaryHeap::new();
    for i in 0..n {
        que.push((arr1[i] + arr2[indices[0]], i, 0));
    }

    while let Some((s, i, j)) = que.pop() {
        if !ban.contains(&(i, indices[j])) {
            println!("{}", s);
            break;
        }
        if j + 1 < n {
            que.push((arr1[i] + arr2[indices[j + 1]], i, j + 1));
        }
    }
}

fn argsort(arr: &Vec<i64>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();
    indices.sort_by_key(|i| arr[*i]);
    indices
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
