A, B, C, K = map(int, input().split())

ans = 0

if K > 0:
    ans += min(K, A) * 1
    K -= A

if K > 0:
    ans += min(K, B) * 0
    K -= B

if K > 0:
    ans += min(K, C) * (-1)
    K -= C

print(ans)