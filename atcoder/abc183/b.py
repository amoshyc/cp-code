sx, sy, gx, gy = map(int, input().split())
dx = gx - sx
dy = -gy - sy
ans = sx + dx * (-sy) / dy
print('{:.15f}'.format(ans))