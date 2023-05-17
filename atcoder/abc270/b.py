X, Y, Z = map(int, input().split())

if 0 <= Y <= X:
    if Y <= Z:
        print(-1)
    elif 0 <= Z <= Y:
        print(abs(X))
    else:
        print(2 * abs(Z) + abs(X))
elif X <= Y <= 0:
    if Z <= Y:
        print(-1)
    elif Y <= Z <= 0:
        print(abs(X))
    else:
        print(2 * abs(Z) + abs(X))
else:
    print(abs(X))