#![allow(unused)]

// [Problem]
// There are N distinct points P1​,...,PN​ on the coordinate plane, and the coordinates of point Pi​ are (Xi​,Yi​).
// From these N points, you want to select distinct three points Pi​,Pj​,Pk​ to maximize the angle ∠Pi​Pj​Pk​. Please output this maximum value in degrees.
// However, we assume that 0∘≤∠Pi​Pj​Pk​≤180∘.

// [Solution]
// Enumerate all the points. For a point (x1, y1), sort all other points via the angle to this point.
// Then enumerate the second point (x2, y2) and binary search the third point (x3, y3) on the remaing point.
// The binary search is finding the point whose angle closest to pi + atan2(y2 - y1, x2 - x1).

use std::f64::consts::TAU; // 2 * pi

fn main() {
    let n = read::<usize>();
    let mut xs = vec![];
    let mut ys = vec![];
    for _ in 0..n {
        let xy = readv::<f64>();
        xs.push(xy[0]);
        ys.push(xy[1]);
    }

    let mut ans: f64 = 0.0;

    let angle_between = |a: f64, b: f64| {
        let d = (a - b).rem_euclid(TAU);
        if d >= TAU / 2.0 {
            TAU - d
        } else {
            d
        }
    };

    for i in 0..n {
        let mut angles = (0..n)
            .filter(|j| *j != i)
            .map(|j| (ys[j] - ys[i]).atan2(xs[j] - xs[i]).rem_euclid(TAU))
            .collect::<Vec<_>>();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for j in 0..angles.len() {
            let opposite = (angles[j] + TAU / 2.0).rem_euclid(TAU);
            let k = angles.partition_point(|x| *x < opposite);
            if k < angles.len() && k != j {
                ans = ans.max(angle_between(angles[j], angles[k]));
            }
            if k >= 1 && k - 1 != j {
                ans = ans.max(angle_between(angles[j], angles[k - 1]));
            }
        }
    }

    println!("{:.8}", ans.to_degrees());
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
