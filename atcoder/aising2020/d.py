import numpy as np
from typing import List, Tuple

N = int(input())
X = list(map(int, input()))


def binary_mod(bits: List[int], div: int) -> int:
    if div == 0:
        return 0

    val = 0
    for b in bits:
        val = (val * 2 + b) % div
    return val


def f(x):
    cnt = 0
    while x > 0:
        x = x % bin(x).count('1')
        cnt += 1
    return cnt


popcount = sum(X)
div1 = popcount + 1
div2 = popcount - 1

if popcount == 0:
    print('\n'.join(map(str, [1] * N)))
else:
    rem1 = binary_mod(X, div1)
    rem2 = binary_mod(X, div2)

    for i in range(N):
        if X[i] == 0:
            r = (rem1 + pow(2, N - 1 - i, div1)) % div1
            print(f(r) + 1)
        else:
            if div2 > 0:
                r = (rem2 - pow(2, N - 1 - i, div2) + div2) % div2
                print(f(r) + 1)
            else:
                print(0)
