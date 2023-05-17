from collections import defaultdict

N = int(input())
A = list(map(int, input().split()))

cnt = defaultdict(int)
for a in A:
    cnt[a] += 1
ttl = sum(A)

Q = int(input())
for _ in range(Q):
    b, c = map(int, input().split())
    
    if b in cnt:
        occ = cnt.pop(b)
        cnt[c] += occ
        ttl = ttl - b * occ + c * occ
    
    print(ttl)