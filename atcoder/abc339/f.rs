#![allow(unused)]

fn main() {
    let mods: Vec<u64> = vec![
        1979983570, 1309031260, 2077485478, 1048067485, 2994731891, 1707276448, 2911151046,
        3078030383, 2412342338, 2092141787,
    ];

    let n = read::<usize>();
    let mut vecs = vec![vec![0; 10]; n];
    for i in 0..n {
        let s = reads();
        let s = mapv(&s, |&c| (c as u32 - '0' as u32) as u64);
        for j in 0..10 {
            let mut m = 0;
            for k in 0..s.len() {
                m = m * 10 % mods[j];
                m = (m + s[k]) % mods[j];
            }
            vecs[i][j] = m;
        }
    }

    let mut cnt = std::collections::HashMap::new();
    for i in 0..n {
        let entry = cnt.entry(vecs[i].clone());
        *entry.or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        let v1 = vecs[i].clone();
        for j in 0..n {
            let v2 = vecs[j].clone();

            let mut v3 = vec![0; 10];
            for k in 0..10 {
                v3[k] = v1[k] * v2[k] % mods[k];
            }
            ans += cnt.get(&v3).unwrap_or(&0);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
