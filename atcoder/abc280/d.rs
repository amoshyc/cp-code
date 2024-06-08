fn main() {
    let k = read::<u64>();

    let sieve = SieveOfEratosthenes::new(1_000_000);
    let factors = sieve.factorize(k);

    let mut ans = 1;
    for &(p, e) in factors.iter() {
        let mut v = 0;
        let mut c = 0;
        while c < e {
            v += p;
            let mut x = v;
            while x % p == 0 {
                x /= p;
                c += 1;
            }
        }
        ans = std::cmp::max(ans, v);
    }

    println!("{}", ans);
}

struct SieveOfEratosthenes {
    primes: Vec<u64>,
}

impl SieveOfEratosthenes {
    fn new(v: usize) -> Self {
        let mut is_prime = vec![true; v + 1];
        let mut primes = vec![];
        for i in 2..=v {
            if is_prime[i] {
                primes.push(i as u64);
                for j in ((i * i)..=v).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        Self { primes }
    }

    fn factorize(&self, mut x: u64) -> Vec<(u64, u64)> {
        assert!(x > 1);
        let mut res = vec![];
        for &p in self.primes.iter() {
            let mut exp = 0;
            while x % p == 0 {
                exp += 1;
                x = x / p;
            }
            if exp > 0 {
                res.push((p, exp))
            }
            if p * p > x {
                break;
            }
        }
        if x > 1 {
            res.push((x, 1));
        }
        res
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
