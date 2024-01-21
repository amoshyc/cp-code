N, M = map(int, input().split())
A = [input() for _ in range(2 * N)]
players = [[0, i] for i in range(2 * N)]

for round in range(M):
    for i in range(N):
        _, u = players[i * 2 + 0]
        _, v = players[i * 2 + 1]
        if (A[u][round], A[v][round]) in [('G', 'C'), ('C', 'P'), ('P', 'G')]:
            players[i * 2 + 0][0] += 1 # winner is u
        if (A[u][round], A[v][round]) in [('C', 'G'), ('P', 'C'), ('G', 'P')]:
            players[i * 2 + 1][0] += 1 # winner is v
    players.sort(key=lambda p: (-p[0], p[1]))

for cnt, i in players:
    print(i + 1)