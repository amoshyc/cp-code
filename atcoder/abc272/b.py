N, M = map(int, input().split())
S = [set() for _ in range(M)]
for i in range(M):
    k, *xs = list(map(int, input().split()))
    for x in xs:
        S[i].add(x)

ans = 'Yes'
for i in range(1, N + 1):
    for j in range(i + 1, N + 1):
        if any([(i in s) and (j in s) for s in S]):
            continue
        else:
            ans = 'No'
print(ans)