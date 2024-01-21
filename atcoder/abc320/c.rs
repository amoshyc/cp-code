#![allow(unused)]

fn main() {
    let m = read::<usize>();
    let mut pos = vec![vec![vec![]; 3]; 10];
    for i in 0..3 {
        let s = reads();
        let s = mapv(&s, |&c| c.to_digit(10).unwrap() as usize);
        for j in 0..m {
            pos[s[j]][i].push(j);
            pos[s[j]][i].push(m + j);
            pos[s[j]][i].push(2 * m + j);
        }
    }

    for v in 0..10 {
        for i in 0..3 {
            pos[v][i].sort();
        }
    }

    let mut ans = !0;
    for v in 0..10 {
        let mut p: Vec<usize> = (0..3).collect();
        loop {
            let mut picks = std::collections::HashSet::new();
            let mut val = 0;
            for i in 0..3 {
                for &x in pos[v][p[i]].iter() {
                    if !picks.contains(&x) {
                        val = val.max(x);
                        picks.insert(x);
                        break;
                    }
                }
            }

            if picks.len() == 3 {
                ans = ans.min(val);
            }

            // println!("{} {:?}: {:?} {}", v, p, picks, val);

            if next_permutation(&mut p).is_none() {
                break;
            }
        }
    }

    if ans == !0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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
