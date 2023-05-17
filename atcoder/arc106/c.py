N, M = map(int, input().split())

if M == 0:
    segs = [(2 * i + 10, 2 * i + 11) for i in range(N)]
    print('\n'.join([f'{l} {r}' for l, r in segs]))
elif 1 <= M <= N - 2:
    N_top = M + 1
    segs = [(2 * i + 10, 2 * i + 11) for i in range(N_top)]
    
    L, R = segs[0][0] - 1, segs[-1][1] + 1
    segs.append((L, R))

    N_rest = N - M - 2
    segs.extend((2 * i + R + 10, 2 * i + R + 11) for i in range(N_rest))

    print('\n'.join([f'{l} {r}' for l, r in segs]))
else:
    print(-1)
