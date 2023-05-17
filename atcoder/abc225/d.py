N, Q = map(int, input().split())
pre = [-1 for _ in range(N + 1)]
nxt = [-1 for _ in range(N + 1)]

for _ in range(Q):
    cmds = input().split()
    if cmds[0] == '1':
        x, y = map(int, cmds[1:])
        nxt[x] = y
        pre[y] = x
    if cmds[0] == '2':
        x, y = map(int, cmds[1:])
        nxt[x] = -1
        pre[y] = -1
    if cmds[0] == '3':
        u = int(cmds[1])
        while pre[u] >= 1:
            u = pre[u]
        ans = [u]
        while nxt[u] >= 1:
            u = nxt[u]
            ans.append(u)
        print('{} {}'.format(len(ans), ' '.join(map(str, ans))))        
