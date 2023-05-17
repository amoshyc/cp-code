A, B, C, D, E, F, X = map(int, input().split())
A, B = B, A
D, E = E, D

cycle1 = X // (B + C)
rest = X - cycle1 * (B + C)
dist1 = A * B * cycle1 + A * min(B, rest)
# print(cycle1, rest, dist1, A * B * cycle1, A * min(B, rest))

cycle2 = X // (E + F)
rest = X - cycle2 * (E + F)
dist2 = D * E * cycle2 + D * min(E, rest)
# print(cycle2, rest, dist2)


if dist1 == dist2:
    print('Draw')
else:
    print('Takahashi' if dist1 > dist2 else 'Aoki')