import math

class SieveOfEratosthenes:
    def __init__(self, V):
        self.is_prime = [True] * (V + 1)
        self.primes = []
        for i in range(2, V + 1):
            if self.is_prime[i]:
                self.primes.append(i)
                for j in range(i * i, V + 1, i):
                    self.is_prime[j] = False
    
    def factorize(self, x):
        assert x > 1
        result = []
        for p in self.primes:
            exp = 0
            while x % p == 0:
                exp += 1
                x = x // p
            if exp > 0:
                result.append((p, exp))
            if p * p > x:
                break
        if x > 1:
            result.append((x, 1))
        return result


def solve():
    N = int(input())
    A = list(map(int, input().split()))

    if all(a == 1 for a in A):
        return 'pairwise coprime'

    gcd_all = 0
    for a in A:
        gcd_all = math.gcd(gcd_all, a)

    if gcd_all > 1:
        return 'not coprime'
    
    sieve = SieveOfEratosthenes(max(A))
    used_primes = set()
    for a in A:
        if a != 1:
            factors = sieve.factorize(a)
            for (p, e) in factors:
                if p in used_primes:
                    return 'setwise coprime'
                used_primes.add(p)
    
    return 'pairwise coprime'

print(solve())