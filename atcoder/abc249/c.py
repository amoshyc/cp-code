from string import ascii_lowercase

def check(S):
    ans = 0
    for c in ascii_lowercase:
        cnt = sum(1 for s in S if c in s)
        if cnt == K:
            ans += 1
    return ans

N, K = map(int, input().split())
S = [input() for _ in range(N)]

ans = -1
for i in range(1 << N):
    SS = [S[j] for j in range(N) if i & (1 << j)]
    ans = max(ans, check(SS))
print(ans)