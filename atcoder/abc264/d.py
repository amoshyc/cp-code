S = list(input())

cnt = 0
for i, c in enumerate('atcoder'):
    while S[i] != c:
        j = S.index(c)
        S[j], S[j - 1] = S[j - 1], S[j]
        cnt += 1
print(cnt)
