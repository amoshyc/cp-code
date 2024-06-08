#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<i32>();

        // dp1[i] = max length of increasing subsequence ending at i
        let (dp1, last) = longest_increasing_subsequence(&arr, true, i32::MAX);

        // dp2[i] = max length of increasing subsequence starting from i
        let inv_rev = (0..n).map(|i| -arr[n - 1 - i]).collect();
        let (mut dp2, _) = longest_increasing_subsequence(&inv_rev, true, i32::MAX);
        dp2.reverse();

        // index i is included in LIS if dp1[i] + dp2[i] > len(LIS)
        let lis = *dp1.iter().max().unwrap();
        let indices: Vec<usize> = (0..n)
            .filter(|&i| dp1[i] + dp2[i] > lis)
            .map(|i| i + 1)
            .collect();

        ans.push(format!("{}\n{}", indices.len(), join(&indices, " ")));
    }

    println!("{}", join(&ans, "\n"));
}

fn longest_increasing_subsequence<T>(arr: &Vec<T>, strict: bool, inf: T) -> (Vec<usize>, Vec<T>)
where
    T: PartialOrd + Clone,
{
    // last[i] = the minimum value of the last element of increasing subsequence of length i
    // dp[i] = maximum length of increasing subsequence ending at i
    let n = arr.len();
    let mut last = vec![inf; n];
    let mut dp = vec![0; n];
    for i in 0..n {
        let j = last.partition_point(|x| {
            if strict {
                *x < arr[i] // strictly LIS
            } else {
                *x <= arr[i] // monotonically LIS
            }
        });
        last[j] = arr[i].clone();
        dp[i] = j + 1;
    }
    (dp, last)
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
