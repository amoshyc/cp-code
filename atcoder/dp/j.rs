#![allow(unused)]

// E(x, y, z) = expected number of operations to finish
// x dishes with 3 suishis and
// y dishes with 2 suishis and
// z dishes with 1 sushi

// E(0, 0, 0) = 0

// To find E(x, y, z), we enumerate the outcome of the die, which has
// x / n probability to get a dish with 3 suishis,
// y / n probability to get a dish with 2 suishis,
// z / n probability to get a dish with 1 suishi,
// (n - x - y - z) / n probability to get a dish with 0 suishi
// that is,
// E(x, y, z) = 1 + x/n E(x - 1, y + 1, z) + y/n E(x, y - 1, z + 1) + z/n E(x, y, z - 1) + (n - x - y - z)/n E(x, y, z)
// move E(x, y, z) to the lhs
// (x + y + z)/n E(x, y, z) = 1 + x/n E(x - 1, y + 1, z) + y/n E(x, y - 1, z + 1) + z/n E(x, y, z - 1)
// E(x, y, z) =
//    x / (x + y + z) E(x - 1, y + 1, z) +
//    y / (x + y + z) E(x, y - 1, z + 1) +
//    z / (x + y + z) E(x, y, z - 1) +
//    n / (x + y + z)

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    
    let mut cnt = vec![0; 4];
    for &x in arr.iter() {
        cnt[x] += 1;
    }

    let sum = cnt[1] + cnt[2] + cnt[3];
    let mut dp = vec![vec![vec![0.0 as f64; sum + 1]; sum + 1]; sum + 1];
    dp[0][0][0] = 0.0;
    for x in 0..=sum {
        for y in 0..=(sum - x) {
            for z in 0..=(sum - x - y) {
                if (x, y, z) == (0, 0, 0) {
                    continue;
                }
                let xyz = (x + y + z) as f64;
                dp[x][y][z] = (n as f64) / xyz;
                if x >= 1 && y + 1 <= sum {
                    dp[x][y][z] += (x as f64) / xyz * dp[x - 1][y + 1][z];
                }
                if y >= 1 && z + 1 <= sum {
                    dp[x][y][z] += (y as f64) / xyz * dp[x][y - 1][z + 1];
                }
                if z >= 1 {
                    dp[x][y][z] += (z as f64) / xyz * dp[x][y][z - 1];
                }
            }
        }
    }

    println!("{:.12}", dp[cnt[3]][cnt[2]][cnt[1]]);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
