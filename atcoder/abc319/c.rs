#![allow(unused)]

fn main() {
    let mut arr = vec![];
    for r in 0..3 {
        arr.push(readv::<usize>());
    }

    let mut cnt = 0;
    let mut ttl = 0;
    let mut p: Vec<usize> = (0..9).collect();
    loop {
        let mut rows = vec![vec![]; 3];
        let mut cols = vec![vec![]; 3];
        let mut diag1 = vec![];
        let mut diag2 = vec![];
        for &x in p.iter() {
            let (r, c) = (x / 3, x % 3);
            rows[r].push(arr[r][c]);
            cols[c].push(arr[r][c]);
            if r == c {
                diag1.push(arr[r][c]);
            }
            if r == 2 - c {
                diag2.push(arr[r][c]);
            }
        }

        let mut ok = true;
        for i in 0..3 {
            if rows[i][0] == rows[i][1] && rows[i][1] != rows[i][2] {
                ok = false;
            }
            if cols[i][0] == cols[i][1] && cols[i][1] != cols[i][2] {
                ok = false;
            }
        }
        if diag1[0] == diag1[1] && diag1[1] != diag1[2] {
            ok = false;
        }
        if diag2[0] == diag2[1] && diag2[1] != diag2[2] {
            ok = false;
        }

        if ok {
            cnt += 1;
        }
        ttl += 1;

        if next_permutation(&mut p).is_none() {
            break;
        }
    }

    println!("{:.8}", cnt as f64 / ttl as f64);
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
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
