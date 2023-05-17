from bisect import bisect_right

N, K = map(int, input().split())
P = [[int(x) for x in input().split()] for _ in range(N)]
S = [sum(p) for p in P]
V = sorted(S)

for s in S:
    rank = N - bisect_right(V, s + 300) + 1
    print('Yes' if rank <= K else 'No')