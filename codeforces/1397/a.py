from collections import defaultdict


def solve():
    N = int(input())

    cnt = defaultdict(int)
    for _ in range(N):
        s = input()
        for c in s:
            cnt[c] += 1

    for k, v in cnt.items():
        if v % N != 0:
            return 'No'
    return 'Yes'


TC = int(input())
for _ in range(TC):
    print(solve())

