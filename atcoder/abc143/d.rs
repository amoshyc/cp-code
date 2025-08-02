#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<usize>();
    arr.sort();

    let mut ans = 0;
    for i in 2..n {
        for j in 1..i {
            // 1) arr[i] + arr[j] > arr[k] => arr[k] < arr[i] + arr[j]
            // 2) arr[i] + arr[k] > arr[j] => arr[k] > arr[j] - arr[j]
            // 3) arr[j] + arr[k] > arr[i] => arr[k] > arr[i] - arr[j]

            // since arr is sorted, we have
            // arr[i] - arr[j] < arr[k] < arr[i] + arr[j]
            // => arr[i] - arr[j] + 1 <= arr[k] <= arr[i] + arr[j] - 1

            // lb..ub
            let lb = arr.partition_point(|x| *x < arr[i] - arr[j] + 1);
            let ub = arr.partition_point(|x| *x <= arr[i] + arr[j] - 1);
            if lb < j {
                let ub = ub.min(j);
                if lb < ub {
                    ans += ub - lb;
                }
            }
        }
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
