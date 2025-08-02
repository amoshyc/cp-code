#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    let mut res = vec![0; tc];
    for tid in 0..tc {
        let n = read::<usize>();
        let s = reads();

        // The final S after operations must has form:
        // 0...01....10....0
        //   A     B     C
        // A part: 0 or more '0'
        // B part: 0 or more '1'
        // C part: 0 or more '0'

        // dp[i, A/B/C] = minimum op to make S[0..=i] valid while the i-th char is in part A/B/C

        // dp[i, A] = dp[i - 1, A] + (1 if S[i] == '1' else 0)
        // dp[i, B] = min(dp[i - 1, A], dp[i - 1, B]) + (1 if S[i] == '0' else 0)
        // dp[i, C] = min(dp[i - 1, A], dp[i - 1, B], dp[i - 1, C]) + (1 if S[i] == '1' else 0)

        let inf = n + 1;
        let mut dp = vec![vec![inf, inf, inf]; n];

        dp[0][0] = (s[0] == '1') as usize;
        dp[0][1] = (s[0] == '0') as usize;
        dp[0][2] = (s[0] == '1') as usize;

        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + (s[i] == '1') as usize;
            dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + (s[i] == '0') as usize;
            dp[i][2] = dp[i - 1][0].min(dp[i - 1][1]).min(dp[i - 1][2]) + (s[i] == '1') as usize;
        }

        res[tid] = dp[n - 1][0].min(dp[n - 1][1]).min(dp[n - 1][2]);
    }

    println!("{}", join(&res, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
