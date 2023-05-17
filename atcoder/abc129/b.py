N = int(input())
W = [int(x) for x in input().split()]
S = sum(W)

ans = abs((S - W[0]) - W[0])

pref = W[0]
for i in range(1, N - 1):
    pref = pref + W[i]
    ans = min(ans, abs((S - pref) - pref))

print(ans)