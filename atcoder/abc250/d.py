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


P = SieveOfEratosthenes(10 ** 6).primes
# print(len(P)) # 78498

N = int(input())
cnt = 0
for i in range(len(P)):
    for j in range(i + 1, len(P)):
        if P[i] * P[j] ** 3 <= N:
            cnt += 1
        else:
            break
print(cnt)
