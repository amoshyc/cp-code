from bisect import bisect_right

N, M, K = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

cumA = [0]
for i in range(N):
    cumA.append(cumA[-1] + A[i])
cumB = [0]
for i in range(M):
    cumB.append(cumB[-1] + B[i])

# print(cumA)
# print(cumB)

ans = 0
for i, s1 in enumerate(cumA):
    j = bisect_right(cumB, K - s1) - 1
    if j >= 0:
        s2 = cumB[j]
        assert s1 + s2 <= K
        ans = max(ans, i + j)

print(ans)

