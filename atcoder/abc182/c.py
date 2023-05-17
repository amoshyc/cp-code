from collections import defaultdict

N = input()
r = int(N) % 3

if r == 0:
    print(0)
    exit()

cnt = defaultdict(int)
for c in N:
    cnt[int(c) % 3] += 1

if cnt[r] > 0 and len(N) > 1:
    print(1)
    exit()

if r == 2 and cnt[1] >= 2 and len(N) > 2:
    print(2)
    exit()

if r == 1 and cnt[2] >= 2 and len(N) > 2:
    print(2)
    exit()

print(-1)

