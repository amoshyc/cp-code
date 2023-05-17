from collections import defaultdict

follow = defaultdict(lambda: set())
N, Q = map(int, input().split())
for _ in range(Q):
    t, a, b = map(int, input().split())
    if t == 1:
        follow[a].add(b)
    if t == 2:
        if b in follow[a]:
            follow[a].remove(b)
    if t == 3:
        if b in follow[a] and a in follow[b]:
            print('Yes')
        else:
            print('No')