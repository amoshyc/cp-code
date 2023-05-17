X, K = map(int, input().split())
for i in range(1, K + 1):
    base = 10 ** i
    if X % base >= base // 2:
        X = (X // base + 1) * base 
    else:
        X = X // base * base
print(X)