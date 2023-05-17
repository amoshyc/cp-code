N, M = map(int, input().split())
A = [input() for _ in range(2 * N)]

players = [[0, i] for i in range(2 * N)]

for round in range(M):
    delta = []
    for i in range(N):
        _, u = players[i * 2 + 0]
        _, v = players[i * 2 + 1]

        if (A[u][round], A[v][round]) in [('G', 'C'), ('C', 'P'), ('P', 'G')]:
            winner = u
        elif (A[u][round], A[v][round]) in [('G', 'G'), ('C', 'C'), ('P', 'P')]:
            winner = -1
        else:
            winner = v

        if winner == u:
            delta.extend([+1, 0])
        elif winner == v:
            delta.extend([0, +1])
        else:
            delta.extend([0, 0])
    
    for i in range(2 * N):
        players[i][0] += delta[i]
    
    players.sort(key=lambda p: (-p[0], p[1]))

for cnt, i in players:
    print(i + 1)