N, M = map(int, input().split())
s = []
for _ in range(N):
    inp = input()
    s.append(set(i for i in range(M) if inp[i] == 'o'))

A = set(range(M))
cnt = 0
for i in range(N):
    for j in range(i + 1, N):
        if s[i] | s[j] == A:
            cnt += 1
print(cnt)