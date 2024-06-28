def solve():
    S = input()
    cnt = 0
    n_remove = 0
    n_rest = 0
    for i, c in enumerate(S):
        if c == 'A':
            cnt += 1
        if c == 'B':
            if cnt > 0:
                cnt -= 1
                n_remove += 1
            else:
                n_rest += 1
    n_rest = n_rest % 2
    return S.count('A') - n_remove + n_rest


TC = int(input())
for _ in range(TC):
    print(solve())