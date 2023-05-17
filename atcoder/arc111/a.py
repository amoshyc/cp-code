'''
floor((10^N) / M) mod M
= (10^N - r) / M mod M
= (10^N - r) / M - k * M
= (10^N - r - k * M^2) / M
= ((10^N - k * M^2) - r) / M
'''

N, M = map(int, input().split())
print(pow(10, N, M * M) // M)
