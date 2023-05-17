N, K = map(int, input().split())
A = [int(x) for x in input().split()]

# print(sum(min(a, 1) for a in A))
# print(sum(min(a, 2) for a in A))

lb, ub = 0, K + 1
while ub - lb > 1:
    m = (lb + ub) // 2

    # the number of eaten apply in cycle [1, m]
    cnt = sum(min(a, m) for a in A)
    
    if cnt > K:
        ub = m
    else:
        lb = m

n_cycle = lb

cnt = 0
for i in range(N):
    if A[i] >= n_cycle:
        cnt += n_cycle
        A[i] -= n_cycle
    else:
        cnt += A[i]
        A[i] = 0

for i in range(N):
    if cnt >= K:
        continue
    if A[i] >= 1:
        A[i] -= 1
        cnt += 1

print(' '.join(map(str, A)))

