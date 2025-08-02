#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let fee = readv::<i64>();
    let mut animals = vec![vec![]; n];
    for animal_id in 0..m {
        let inp = readv::<usize>();
        for zoo in &inp[1..] {
            let zoo = zoo - 1;
            animals[zoo].push(animal_id);
        }
    }

    let mut ans = i64::MAX;
    for mask in 0..(1 << n) {
        for submask in submask_iter(mask) {
            let mut cnt = vec![0; m];
            let mut sum = 0;

            for zoo in 0..n {
                let mut times = 0;
                times += (mask >> zoo) & 1;
                times += (submask >> zoo) & 1;
                if times > 0 {
                    sum += times as i64 * fee[zoo];
                    for &animal in &animals[zoo] {
                        cnt[animal] += times;
                    }
                }
            }

            if (0..m).all(|i| cnt[i] >= 2) {
                ans = ans.min(sum);
            }
        }
    }

    println!("{}", ans);
}

fn submask_iter(mask: usize) -> impl std::iter::Iterator<Item = usize> {
    let mut submask = mask;
    let iter1 = std::iter::once(submask);
    let iter2 = std::iter::from_fn(move || {
        if submask == 0 {
            None
        } else {
            submask = (submask - 1) & mask;
            Some(submask)
        }
    });
    iter1.chain(iter2)
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
