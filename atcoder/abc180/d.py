X, Y, A, B = map(int, input().split())

ans = 0
while True:
    if min(X * A, X + B) >= Y:
        break
    k = (min(X * A, Y - 1) - X) // B
    if k >= 1:
        X = X + k * B
        ans += k
    else:
        X = X * A
        ans += 1
print(ans)
