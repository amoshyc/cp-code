N, A, B = map(int, input().split())

q, r = divmod(N, A + B)
ans = 0
ans += q * A
ans += min(r, A)

print(ans)