S, T, X = map(int, input().split())
if X < S:
    X += 24
if T < S:
    T += 24
print('Yes' if X >= S and X < T else 'No')