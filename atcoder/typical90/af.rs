#![allow(unused)]

// [Problem]
// The ABC Track and Field Club has N relay race runners. In a relay race, each runner is responsible for running one leg of the race. No multiple runners can run the same leg, and no runner can run multiple legs. The relay course consists of sections from the 1st to the Nth leg, and the time it takes for runner i to run leg j is Ai,j.
// The relay race proceeds with runners running in order from the 1st leg to the Nth leg. The runner who finishes leg j (1 ≤ j ≤ N - 1) must pass the sash (tasuki) to the runner of leg j+1. The time taken for this sash exchange is negligibly short and can be ignored. The race ends when the runner of the Nth leg crosses the finish line with the sash.
// However, there are M rumors in the ABC Track and Field Club. The i-th rumor is that "runner Xi and runner Yi do not get along." The runners mentioned in the rumors cannot exchange the sash directly. In other words, runner Xi cannot immediately precede runner Yi, and runner Yi cannot immediately precede runner Xi.
// Determine the minimum time required for the ABC Track and Field Club to finish the relay race. If it is not possible to complete the race regardless of how the legs are assigned to the runners, output -1 instead.

// [Solution]
// Since N is small, inspect all permutation.

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for r in 0..n {
        arr.push(readv::<i64>());
    }

    let mut ban = vec![vec![false; n]; n];
    let m = read::<usize>();
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        ban[u][v] = true;
        ban[v][u] = true;
    }

    let mut ans = i64::MAX;
    let mut perm: Vec<usize> = (0..n).collect();
    loop {
        let mut total = 0;
        let mut valid = true;
        for i in 0..n {
            total += arr[perm[i]][i];
            if i + 1 < n {
                valid &= !ban[perm[i]][perm[i + 1]];
            }
        }

        if valid {
            ans = ans.min(total);
        }

        if next_permutation(&mut perm).is_none() {
            break;
        }
    }

    if ans == i64::MAX {
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
