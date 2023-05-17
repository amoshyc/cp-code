val = [-1]
par = [0]
cur = 0
notebook = {}

Q = int(input())
ans = []
for _ in range(Q):
    cmd, *v = input().split()
    if cmd == 'ADD':
        v = int(v[0])
        val.append(v)
        par.append(cur)
        cur = len(val) - 1
    if cmd == 'DELETE':
        cur = par[cur]
    if cmd == 'SAVE':
        idx = int(v[0])
        notebook[idx] = cur
    if cmd == 'LOAD':
        idx = int(v[0])
        cur = notebook.get(idx, 0)
    ans.append(val[cur])

print(' '.join(map(str, ans)))

