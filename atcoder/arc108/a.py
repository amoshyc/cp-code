S, P = map(int, input().split())

for n in range(10**6 + 1):
    m = S - n
    if n >= 1 and m >= 1 and n * m == P:
        print('Yes')
        break
else:
    print('No')