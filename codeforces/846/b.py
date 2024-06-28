from heapq import *

N, K, M = map(int, input().split())
T = list(map(int, input().split()))

def rest(m, n):
    pq = []
    for i in range(n):
        for j in range(K):
            heappush(pq, T[j])
    
    cnt = 0
    while len(pq) > 0 and m >= pq[0]:
        subtask = heappop(pq)
        m -= subtask
        cnt += 1
    return cnt

ans = -1
for i in range(N + 1):
    time_ = M - sum(T) * i
    if time_ < 0: continue
    score = (1 * i) + (K * i) + rest(time_, N - i)

    # print(i, time_, score, rest(time_, N - i))

    ans = max(ans, score)

print(ans)