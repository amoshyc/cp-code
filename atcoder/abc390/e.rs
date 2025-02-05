#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, x) = (inp[0], inp[1]);
    let mut costs = vec![vec![]; 3];
    let mut values = vec![vec![]; 3];
    for _ in 0..n {
        let vac = readv::<usize>();
        let (v, a, c) = (vac[0] - 1, vac[1] as i64, vac[2]);
        values[v].push(a);
        costs[v].push(c);
    }

    // dp0/1/2[j] = maximum total vitamin using total cost j
    let dp0 = knapsack_01(&values[0], &costs[0], x);
    let dp1 = knapsack_01(&values[1], &costs[1], x);
    let dp2 = knapsack_01(&values[2], &costs[2], x);

    // Does the total cost needed to get at least m for all vitamin 0/1/2 <= x
    // = min_cost(vitamin 0) + min_cost(vitamin 1) + min_cost(vitamin 2) <= x
    // where min_cost can be found by lower_bound(dp, m)
    let ok = |m: i64| -> bool {
        let cost0 = dp0.partition_point(|v| *v < m);
        let cost1 = dp1.partition_point(|v| *v < m);
        let cost2 = dp2.partition_point(|v| *v < m);
        cost0 + cost1 + cost2 <= x
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 10i64.pow(10);
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{}", lb);
}

// dp[j] = maximum total value using total cost j
fn knapsack_01(values: &Vec<i64>, costs: &Vec<usize>, max: usize) -> Vec<i64> {
    assert!(values.len() == costs.len());
    let mut dp = vec![0; max + 1];
    for (v, c) in values.iter().zip(costs.iter()) {
        for j in (0..=max).rev() {
            if j + c <= max {
                dp[j + c] = dp[j + c].max(dp[j] + v);
            }
        }
    }
    dp
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
