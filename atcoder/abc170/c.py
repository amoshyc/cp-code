X, N = map(int, input().split())
if N == 0:
    print(X)
else:
    A = list(map(int, input().split()))
    P = list(set(range(-100, +200)) - set(A))
    P = sorted(P)
    print(min(P, key=lambda p: abs(p - X)))