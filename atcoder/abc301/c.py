from string import ascii_lowercase
from collections import defaultdict


def solve():
    S = input()
    T = input()

    cnt_s = defaultdict(int)
    cnt_t = defaultdict(int)
    for c in S:
        cnt_s[c] += 1
    for c in T:
        cnt_t[c] += 1

    for c in ascii_lowercase:
        if cnt_s[c] > cnt_t[c]:
            need = cnt_s[c] - cnt_t[c]
            if c not in 'atcoder' or cnt_t['@'] < need:
                return False
            cnt_t['@'] -= need
        if cnt_s[c] < cnt_t[c]:
            need = cnt_t[c] - cnt_s[c]
            if c not in 'atcoder' or cnt_s['@'] < need:
                return False
            cnt_s['@'] -= need
    
    return True

print('Yes' if solve() else 'No')