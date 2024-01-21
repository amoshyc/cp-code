import itertools

H, W, K = map(int, input().split())
A = [[int(x) for x in input()] for _ in range(H)]

def solve(blocks):
    n_block = len(blocks)
    n_cut = n_block - 1
    sums = [0 for _ in range(n_block)]

    for c in range(W):
        adds = [block[c] for block in blocks]
        if any(a > K for a in adds):
            return H * W
        
        sums = [s + a for s, a in zip(sums, adds)]
        if any(s > K for s in sums):
            n_cut += 1
            sums = adds
    
    return n_cut

ans = H * W
for mask in itertools.product([0, 1], repeat=H - 1):
    mask = [1] + list(mask) + [1]
    pivots = [r for r in range(H + 1) if mask[r]]
    blocks = [A[p1:p2] for p1, p2 in zip(pivots[:-1], pivots[1:])]
    blocks = [[sum(row[c] for row in block) for c in range(W)] for block in blocks]
    ans = min(ans, solve(blocks))
print(ans)
