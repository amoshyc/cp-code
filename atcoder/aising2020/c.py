from itertools import combinations_with_replacement

N = int(input())
ans = [0] * (N + 1)            

for x, y, z in combinations_with_replacement(list(range(1, 100)), 3):
    val = x ** 2 + y ** 2 + z ** 2 + x * y + y * z + x * z

    n_diff = len(set([x, y, z]))
    if n_diff == 3:
        cnt = 6
    if n_diff == 2:
        cnt = 3
    if n_diff == 1:
        cnt = 1

    # print(val)
    
    if val <= N:
        ans[val] += cnt

print('\n'.join(map(str, ans[1:])))
