#![allow(unused)]

// [Problem]
// There is a grid of H rows and W columns.
// At the cell (i, j), where i ranges from 1 to H and j ranges from 1 to W, an integer Ai,j​ is written.
// For all cells (i, j) (1≤i≤H, 1≤j≤W), please find the sum of all integers written in the cells that are in the same row or column as cell (i, j) (including the cell itself).

// [Solution]
// Precalculate the row sum and column sum as R[i] and C[i].
// Then the answer at (r, c) is R[r] + C[c] - A[r][c] since A[r][c] is counted twice.

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(readv::<i64>());
    }

    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];
    for r in 0..h {
        for c in 0..w {
            row_sum[r] += arr[r][c];
            col_sum[c] += arr[r][c];
        }
    }

    let mut ans = vec![vec![0; w]; h];
    for r in 0..h {
        for c in 0..w {
            ans[r][c] = row_sum[r] + col_sum[c] - arr[r][c];
        }
    }

    for r in 0..h {
        println!("{}", join(&ans[r], " "));
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
    read::<String>().chars().collect::<_>()
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
