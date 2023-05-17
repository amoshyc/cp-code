import numpy as np

N, K = map(int, input().split())
A = list(map(int, input().split()))
M = 10**9 + 7

def prod(X):
    ans = 1
    for x in X:
        ans = ans * x % M
    return ans

def solve():
    # Case 1: N == K
    if N == K:
        return prod(A)

    # Case 2: ans is negative
    if all(a < 0 for a in A) and K % 2 == 1:
        return prod(sorted(A)[-K:])

    # Case 3: ans is positive
    posA = np.int64(sorted([a for a in A if a >= 0]))
    negA = np.int64(sorted([a for a in A if a < 0]))

    ans = 1
    if K % 2 == 1: # Keep the largest positive
        ans = ans * posA[-1] % M
        posA = posA[:-1]
    
    # Construct largest value by pairing items
    if len(posA) % 2 == 1: # Discard the one with smallest absolute value
        posA = posA[1:]
    if len(negA) % 2 == 1: # Discard the one with smallest absolute value
        negA = negA[:-1]
    pos_pairs = posA[0::2] * posA[1::2]
    neg_pairs = negA[0::2] * negA[1::2]
    pairs = np.concatenate([pos_pairs, neg_pairs], axis=0).tolist()
    pairs = sorted(pairs, reverse=True)[:K//2]
    ans = ans * prod(pairs) % M

    return ans

print(solve())
        
