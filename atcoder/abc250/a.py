H, W = map(int, input().split())
R, C = map(int, input().split())
R, C = R - 1, C - 1

cnt = 0
for r in range(H):
    for c in range(W):
        if abs(R - r) + abs(C - c) == 1:
            cnt += 1
print(cnt)