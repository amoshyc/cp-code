ans = dict()
for a in range(1, 38):
    for b in range(1, 26):
        ans[pow(3, a) + pow(5, b)] = f'{a} {b}'

N = int(input())
print(ans.get(N, -1))