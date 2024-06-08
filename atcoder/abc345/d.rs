#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, h, w) = (inp[0], inp[1], inp[2]);
    let mut tiles = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        let (a, b) = (inp[0], inp[1]);
        tiles.push((a, b));
        tiles.push((b, a));
    }

    let mut arr = vec![vec![true; w]; h];
    let mut msk = vec![true; tiles.len()];
    let mut ans = false;
    dfs(&mut arr, &mut msk, &tiles, &mut ans);

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(
    arr: &mut Vec<Vec<bool>>,
    mask: &mut Vec<bool>,
    tiles: &Vec<(usize, usize)>,
    ans: &mut bool,
) {
    let h = arr.len();
    let w = arr[0].len();

    let mut curr_r = 0;
    let mut curr_c = 0;
    if let Some((r, c)) = cartesian(0..h, 0..w).find(|&(r, c)| arr[r][c]) {
        curr_r = r;
        curr_c = c;
    } else {
        *ans = true;
        return;
    }

    for i in 0..tiles.len() {
        if !mask[i] {
            continue;
        }
        let (a, b) = tiles[i];

        let mut ok = curr_r + a <= h && curr_c + b <= w;
        if ok {
            // all empty?
            for dr in 0..a {
                for dc in 0..b {
                    if !arr[curr_r + dr][curr_c + dc] {
                        ok = false;
                    }
                }
            }
        }

        if ok {
            for dr in 0..a {
                for dc in 0..b {
                    arr[curr_r + dr][curr_c + dc] = false;
                }
            }
            mask[i] = false;
            mask[i ^ 1] = false;
            dfs(arr, mask, tiles, ans);
            mask[i ^ 1] = true;
            mask[i] = true;
            for dr in 0..a {
                for dc in 0..b {
                    arr[curr_r + dr][curr_c + dc] = true;
                }
            }
        }
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

fn cartesian<T, R1, R2>(r1: R1, r2: R2) -> impl Iterator<Item = (T, T)>
where
    T: Clone,
    R1: std::ops::RangeBounds<T> + Iterator<Item = T> + Clone,
    R2: std::ops::RangeBounds<T> + Iterator<Item = T> + Clone,
{
    r1.flat_map(move |x| r2.clone().map(move |y| (x.clone(), y)))
}
