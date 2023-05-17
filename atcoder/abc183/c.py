from itertools import permutations

N, K = map(int, input().split())
T = [list(map(int, input().split())) for _ in range(N)]

ans = 0
for perm in permutations(range(N)):
    if perm[0] != 0:
        continue
    need = sum(T[perm[i - 1]][perm[i]] for i in range(1, N)) + T[perm[-1]][0]
    if need == K:
        ans += 1
print(ans)