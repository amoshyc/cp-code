import itertools

H, W, K = map(int, input().split())
A = [[int(x) for x in input()] for _ in range(H)]


def solve(blocks):
    n_vert_cut = 0
    n_block = len(blocks)
    vals = [0 for _ in range(n_block)]

    for c in range(W):
        adds = [block[c] for block in blocks]
        if any(add > K for add in adds):
            return H * W

        sums = [val + add for val, add in zip(vals, adds)]
        if any(x > K for x in sums):
            n_vert_cut += 1
            vals = adds
        else:
            vals = sums

    return n_vert_cut + n_block - 1


ans = H * W
for mask in itertools.product([0, 1], repeat=H - 1):
    mask = [1] + list(mask) + [1]
    pivots = [r for r in range(H + 1) if mask[r]]
    blocks = [A[p1:p2] for p1, p2 in zip(pivots[:-1], pivots[1:])]
    blocks = [[sum(row[c] for row in block) for c in range(W)] for block in blocks]
    # print(blocks, '->', solve(blocks))
    ans = min(ans, solve(blocks))
print(ans)
