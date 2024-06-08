from atcoder.segtree import SegTree


def op(a, b):
    res = []
    arr1 = [a[0], a[1], (0, 0)]
    arr2 = [b[0], b[1], (0, 0)]
    i, j = 0, 0
    while i < 3 and j < 3 and len(res) < 2:
        if arr1[i][0] > arr2[j][0]:
            res.append(arr1[i])
            i += 1
        elif arr1[i][0] < arr2[j][0]:
            res.append(arr2[j])
            j += 1
        else:
            res.append((arr1[i][0], arr1[i][1] + arr2[j][1]))
            i += 1
            j += 1
    if len(res) == 1:
        return (res[0], (0, 0))
    else:
        return (res[0], res[1])


N, Q = map(int, input().split())
A = [int(x) for x in input().split()]
B = [((a, 1), (0, 0)) for a in A]

seg = SegTree(op, ((0, 0), (0, 0)), B)

for _ in range(Q):
    cmd = [int(x) for x in input().split()]
    if cmd[0] == 1:
        p, x = (cmd[1] - 1, cmd[2])
        seg.set(p, ((x, 1), (0, 0)))
    else:
        l, r = (cmd[1] - 1, cmd[2])
        ((_, _), (_, ans)) = seg.prod(l, r)
        print(ans)
