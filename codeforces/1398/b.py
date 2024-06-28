def solve():
    S = input()

    vals = []
    
    s, t = 0, 0
    while s < len(S):
        if S[s] == '0':
            s += 1
            continue
        t = s
        while t < len(S) and S[t] == '1':
            t += 1
        vals.append(t - s)
        s = t
    
    vals = sorted(vals, reverse=True)
    return sum(vals[::2])

TC = int(input())
for _ in range(TC):
    print(solve())