#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let sr = reads();
    let sc = reads();

    let mut state = vec![vec!['.'; n]; n];
    if dfs(0, &mut state, &sr, &sc) {
        println!("Yes");
        for r in 0..n {
            println!("{}", join(&state[r], ""));
        }
    } else {
        println!("No");
    }
}

fn dfs(r: usize, state: &mut Vec<Vec<char>>, sr: &Vec<char>, sc: &Vec<char>) -> bool {
    let n = state.len();
    if r == n {
        // println!("---");
        // for r in 0..n {
        //     println!("{}", join(&state[r], ""));
        // }
        for c in 0..n {
            let mut col: Vec<char> = (0..n).map(|i| state[i][c]).filter(|&x| x != '.').collect();
            let mut ok = true;
            ok &= col.len() > 0 && col[0] == sc[c];
            col.sort();
            ok &= col == vec!['A', 'B', 'C'];
            if !ok {
                return false;
            }
        }
        return true;
    }

    let mut line = vec!['A', 'B', 'C'];
    for i in 3..n {
        line.push('.');
    }
    line.sort();
    loop {
        let first = *line.iter().filter(|&x| *x != '.').next().unwrap();
        if first == sr[r] {
            state[r] = line.clone();
            if dfs(r + 1, state, sr, sc) {
                return true;
            }
            state[r].fill('.');
        }
        if next_permutation(&mut line).is_none() {
            break;
        }
    }

    false
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
