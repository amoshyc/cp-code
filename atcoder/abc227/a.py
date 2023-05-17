N, K, A = map(int, input().split())
A = A - 1
print((A + K - 1) % N + 1)