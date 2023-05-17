N = int(input())

# ans = 0
# for v in range(1, N + 1):
#     if v > N // 2:
#         ans += v
#     else:
#         ans += (v + (N // v) * v) * (N // v) // 2
# print(ans)

ans = 0
for v in range(1, N // 2 + 1):
    ans += (v + (N // v) * v) * (N // v) // 2
ans += (N // 2 + 1 + N) * (N - N // 2) // 2
print(ans)