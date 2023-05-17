from heapq import heappush, heappop

N = int(input())
A = list(map(int, input().split()))
A = sorted(A, reverse=True)

pq = [-A[0]]
ans = 0

for a in A[1:]:
    v = -heappop(pq)
    ans += v
    heappush(pq, -a)
    heappush(pq, -a)

print(ans)

