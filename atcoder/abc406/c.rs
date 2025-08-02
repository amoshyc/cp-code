#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pos1 = vec![]; // ^
    let mut pos2 = vec![]; // v
    for i in 1..(n - 1) {
        if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
            pos1.push(i);
        }
        if arr[i] < arr[i - 1] && arr[i] < arr[i + 1] {
            pos2.push(i);
        }
    }

    // for each left endpoint, find the number of valid right endpoint
    let mut ans = 0;
    for i in 0..(n - 1) {
        if arr[i] > arr[i + 1] {
            continue;
        }

        let p = pos1.partition_point(|x| *x <= i);
        let q = pos2.partition_point(|x| *x <= i);
        if p == pos1.len() || q == pos2.len() {
            continue;
        }
        let lb = pos1[p].max(pos2[q]) + 1;

        let end = n - 1;
        let ub1 = pos1.get(p + 1).unwrap_or(&end);
        let ub2 = pos2.get(q + 1).unwrap_or(&end);
        let ub = ub1.min(ub2);

        ans += ub - lb + 1;
    }

    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
