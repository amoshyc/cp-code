fn main() {
    let k = read::<u64>();
    
    let sieve = SieveOfEratosthenes::new(1_000_000);
    let factors = sieve.factorize(k);

    let mut need = std::collections::HashMap::<u64, u64>::new();
    let mut max_prime_over = 1;
    for &(p, e) in factors.iter() {
        need.insert(p, e);
        if e == 1 && p >= 1_000_000 {
            max_prime_over = std::cmp::max(max_prime_over, p);
        }
    }

    if max_prime_over != 1 {
        println!("{}", max_prime_over);
        return;
    }

    // println!("{:?}", need);
    
    for i in 2..=k {
        for (p, e) in sieve.factorize(i) {
            if need.contains_key(&p) {
                need.entry(p).and_modify(|x| *x = (*x).saturating_sub(e));
                if need[&p] == 0 {
                    need.remove(&p);
                }
            }
        }
        
        // println!("{} {:?}", i, need);
        if need.len() == 0 {
            println!("{}", i);
            break;
        }
    }
}


struct SieveOfEratosthenes {
    primes: Vec<u64>,
}

impl SieveOfEratosthenes {
    fn new(max_val: usize) -> SieveOfEratosthenes {
        let mut is_prime = vec![true; max_val + 1];
        let mut primes = vec![];
        for i in 2..=max_val {
            if is_prime[i] {
                primes.push(i as u64);
                let mut j = i * i;
                while j <= max_val {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        SieveOfEratosthenes { primes }
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