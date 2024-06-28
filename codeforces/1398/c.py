from collections import defaultdict

def solve():
    N = int(input())
    A = list(map(int, input()))
    
    P = [A[0]]
    for i in range(1, N):
        P.append(P[-1] + A[i])
    
    ans = 0
    cnt = defaultdict(int)
    for i in range(N - 1, -1, -1):
        q = P[i] - i
        cnt[q] += 1
        ans += cnt[q - A[i] + 1]
    return ans


TC = int(input())
for _ in range(TC):
    print(solve())
