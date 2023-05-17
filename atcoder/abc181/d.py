'''
516 should be No
'''

from collections import defaultdict

S = input()

cnt = defaultdict(int)
for c in S:
    cnt[c] += 1

for x in range(8, 1000, 8):
    strx = str(x).ljust(min(len(S), 3), '0')
    need = defaultdict(int)
    for c in strx:
        need[c] += 1
    if all(cnt[c] >= need[c] for c in need.keys()):
        print('Yes')
        break
else:
    print('No')

