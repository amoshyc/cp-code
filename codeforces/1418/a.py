'''
Using math.ceil is incorrect due to inaccuracy of double
Need k torch = need k sticks + need k coals = need k + yk sticks
Assume first trade is operated n times,
    => 1 - n + xn >= k + yk
    => n >= (k + yk - 1) / (x - 1)
'''


def ceil(a, b):
    return (a + b - 1) // b


tc = int(input())
for _ in range(tc):
    x, y, k = map(int, input().split())
    n_first = ceil((k + y * k - 1), (x - 1))
    n_second = k
    print(n_first + n_second)
