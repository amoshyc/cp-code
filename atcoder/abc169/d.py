from math import sqrt, ceil, floor

def factorize(x):
    result = []
    for p in range(2, floor(sqrt(x))):
        exp = 0
        while x % p == 0:
            exp += 1
            x = x // p
        if exp > 0:
            result.append((p, exp))
    if x > 1:
        result.append((x, 1))
    return result

N = int(input())
factors = factorize(N)
ans = sum([floor((sqrt(exp * 8 + 1) - 1) / 2) for p, exp in factors])
print(ans)
