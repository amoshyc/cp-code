dp = {0: 1}

def f(k):
    if k in dp:
        return dp[k]
    dp[k] = f(k // 2) + f(k // 3)
    return dp[k]


N = int(input())
print(f(N))
