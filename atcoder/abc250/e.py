N = int(input())
HA = [hash(x) for x in input().split()]
HB = [hash(x) for x in input().split()]

PA, PB = [HA[0]], [HB[0]]
SA, SB = set([HA[0]]), set([HB[0]])
for i in range(1, N):
    if HA[i] in SA:
        PA.append(PA[-1])
    else:
        PA.append(PA[-1] ^ HA[i])
        SA.add(HA[i])

    if HB[i] in SB:
        PB.append(PB[-1])
    else:
        PB.append(PB[-1] ^ HB[i])
        SB.add(HB[i])
        

Q = int(input())
for _ in range(Q):
    x, y = map(int, input().split())
    x, y = x - 1, y - 1
    print('Yes' if PA[x] == PB[y] else 'No')
