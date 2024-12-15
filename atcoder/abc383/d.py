from bisect import bisect_left


def sieve_of_eratosthenes(v):
    primes = []
    is_prime = [True for _ in range(v + 1)]
    for i in range(2, v + 1):
        if is_prime[i]:
            primes.append(i)
            for j in range(i * i, v + 1, i):
                is_prime[j] = False
    return primes


# if x has 9 divisors, x can only be one of following forms:
# 1. x = p^8
# 2. x = p^2 q^2

# For case 1, simply inspect all the primes.
# For case 2, wlog, assume (p >= q), inspect p and binary search the primes to find the number of q

N = int(input())

ans = 0
primes = sieve_of_eratosthenes(2_000_000)

# Case 1
for p in primes:
    if p**8 <= N:
        ans += 1
    else:
        break

# Case 2
for i, p in enumerate(primes):
    # number of q such that p * p * q * q <= N
    # = index of the q such that p * p * q * q > N
    # = 0 0 0 1 1 1
    #         ^
    j = bisect_left(primes, True, key=lambda q: p * p * q * q > N)
    ans += min(j, i)
    if j == 0:
        break

print(ans)
