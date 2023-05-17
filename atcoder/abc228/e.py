N, K, M = map(int, input().split())
mod = 998244353
if M % mod != 0: # premise of fermat's little theorem
    X = pow(K, N, mod - 1)
    print(pow(M, X, mod))
else:
    print(0)