V = 400000
N = int(input())
adj = [[] for _ in range(V)]
for _ in range(N):
    a, b = map(int, input().split())
    a, b = a - 1, b - 1
    adj[a].append(b)
    adj[b].append(a)

index = [-1 for _ in range(V)]
answer = 0
for root in range(V):
    if len(adj[root]) == 0:
        continue
    if index[root] != -1:
        continue

    count = 1
    has_back_edge = False
    index[root] = 0
    stack = [(root, -1)]
    while len(stack) > 0:
        u, p = stack.pop()
        for v in adj[u]:
            if v == p:
                continue
            if index[v] == -1:
                index[v] = index[u] + 1  # forward edge
                stack.append((v, u))
                count += 1
            elif index[v] <= index[u]:  # back edge
                has_back_edge = True

    answer += count if has_back_edge else count - 1

print(answer)

