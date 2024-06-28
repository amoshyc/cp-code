#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (n, k, q) = (inp[0] as i64, inp[1], inp[2]);
        let mut arr_a = readv::<i64>();
        let mut arr_b = readv::<i64>();
        arr_a.insert(0, 0);
        arr_b.insert(0, 0);

        let mut ans = vec![];
        for qid in 0..q {
            let d = read::<i64>();
            if d == n {
                ans.push(arr_b[k]);
            } else {
                let p = arr_a.partition_point(|x| *x <= d) - 1;
                let x1 = d - arr_a[p];
                let x2 = arr_a[p + 1] - d;
                let y1 = arr_b[p];
                let y2 = arr_b[p + 1];
                let t = (y1 * x2 + y2 * x1) / (x1 + x2);
                ans.push(t);
            }
        }

        println!("{}", join(&ans, " "));
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
