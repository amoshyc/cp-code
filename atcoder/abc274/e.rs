#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut xy = vec![(0.0, 0.0)];
    for _ in 0..n {
        let inp = readv::<f64>();
        xy.push((inp[0], inp[1]));
    }
    for _ in 0..m {
        let inp = readv::<f64>();
        xy.push((inp[0], inp[1]));
    }

    let dis = |i: usize, j: usize, v: u32| -> f64 {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[j];
        let d = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
        d / (v as f64)
    };

    // 
    // dp[S][u] = min(dp[S - {u}][v] + dis(v, u) for all v)
    let mut dp = vec![vec![1e18 as f64; 1 + n + m]; (1 << (1 + n + m))];
    dp[(1 << 0)][0] = 0.0;
    for s in 0..((1 as usize) << (1 + n + m)) {
        for u in 1..(1 + n + m) {
            if (s & (1 << u)) == 0 {
                continue;
            }
            for v in 0..(1 + n + m) {
                if v == u || (s & (1 << v)) == 0 {
                    continue;
                }
                let prev_s = s ^ (1 << u);
                let vel = (2 as u32).pow((prev_s >> (1 + n)).count_ones());
                let val = dp[prev_s][v] + dis(v, u, vel);
                dp[s][u] = dp[s][u].min(val);
            }
        }
    }

    let mut ans: f64 = 1e18;
    for s in 0..(1_usize << (1 + n + m)) {
        if (1..n + 1).any(|x| s & (1 << x) == 0) {
            continue;
        }

        for u in 1..(1 + n + m) {
            if (s & (1 << u)) == 0 {
                continue;
            }
            let vel = (2 as u32).pow((s >> (1 + n)).count_ones());
            let val = dp[s][u] + dis(u, 0, vel);
            ans = ans.min(val);
        }
    }

    if ans >= 1e18 - 1.0 {
        println!("-1");
    } else {
        println!("{:.7}", ans);
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
