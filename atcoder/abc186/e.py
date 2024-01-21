def extgcd(a, b):
    if b == 0:
        return 1, 0, a
    x1, y1, g = extgcd(b, a % b)
    return y1, x1 - a // b * y1, g


T = int(input())
for _ in range(T):
    N, S, K = map(int, input().split())
    
    # Find minimum x (x >= 0) such that
    # S + xK = 0 (mod N)
    # xK + yN = -S
    # <-> ax + by = m
    # x = m/g x0 + k b/g = p x0 + k q where k is int
    # y = m/g y0 - k a/g = p y0 - k r where k is int

    a, b, m = K, N, -S
    x0, y0, g = extgcd(a, b)
    if m % g != 0:
        print(-1)
    else:
        p, q, r = m // g, b // g, a // g
        if p * x0 < 0:
            # k = ceil(abs(p * x0) / q)
            k = (abs(p * x0) + (q - 1)) // q
        else:
            # k = -floor(abs(p * x0) / q)
            k = -(abs(p * x0) // q)
        x = p * x0 + k * q
        y = p * y0 - k * r
        print(x)

