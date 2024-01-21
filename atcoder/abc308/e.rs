#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let str = reads();

    // M: pref[0], pref[1], pref[2]
    let mut pref = vec![vec![0; n]; 3];
    if str[0] == 'M' {
        pref[arr[0]][0] = 1;
    }
    for i in 1..n {
        pref[0][i] = pref[0][i - 1];
        pref[1][i] = pref[1][i - 1];
        pref[2][i] = pref[2][i - 1];
        if str[i] == 'M' {
            pref[arr[i]][i] += 1;
        }
    }

    // X: suff[0], suff[1], suff[2]
    let mut suff = vec![vec![0; n]; 3];
    if str[n - 1] == 'X' {
        suff[arr[n - 1]][n - 1] = 1;
    }
    for i in (0..(n - 1)).rev() {
        suff[0][i] = suff[0][i + 1];
        suff[1][i] = suff[1][i + 1];
        suff[2][i] = suff[2][i + 1];
        if str[i] == 'X' {
            suff[arr[i]][i] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if str[i] == 'E' {
            for u in 0..3 {
                for v in 0..3 {
                    let vals = vec![u, arr[i], v];
                    let mex = (0..4).filter(|x| !vals.contains(x)).next().unwrap();
                    let cnt_m = pref[u][i];
                    let cnt_x = suff[v][i];
                    ans += mex * cnt_m * cnt_x;
                }
            }
        }
    }

    println!("{}", ans);
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
