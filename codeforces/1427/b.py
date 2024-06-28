def solve():
    n, k = map(int, input().split())
    A = input()

    segs = []
    s, t = 0, 0

    while s < n and A[s] == 'L':
        s += 1
    head = (0, s)

    nn = n
    while nn >= 1 and A[nn - 1] == 'L':
        nn -= 1
    tail = (nn, n)
    
    while s < nn:
        if A[s] == 'W':
            s += 1
            continue
        t = s
        while t < nn and A[t] == 'L':
            t += 1
        segs.append((s, t))
        s = t
    
    segs.sort(key=lambda x: x[1] - x[0])

    B = list(A)
    for (s, t) in segs:
        if k <= 0:
            break
        w = min(t - s, k)
        B[s:s+w] = 'W' * w
        k -= w
    
    if k > 0 and tail[0] != n:
        s, t = tail
        w = min(t - s, k)
        B[s:s+w] = 'W' * w
        k -= w
    if k > 0 and head[1] > 0:
        s, t = head
        w = min(t - s, k)
        B[t - w: t] = 'W' * w
        k -= w
        
    score = 0
    for i in range(n):
        if i >= 1 and B[i - 1] == 'W' and B[i] == 'W':
            score += 2
            continue
        if B[i] == 'W':
            score += 1
            continue
    return score


TC = int(input())
for _ in range(TC):
    print(solve())
 