def ceil_div(a, b):
    return (a + b - 1) // b


N, M = map(int, input().split())
A = sorted([int(x) for x in input().split()])

if M == 0:
    print(1)
    exit()

segs = []
if A[0] != 1:
    segs.append((0, A[0]))
if A[-1] != N:
    segs.append((A[-1], N + 1))
for i in range(1, M):
    if A[i] != A[i - 1] + 1:
        segs.append((A[i - 1], A[i]))

width = min([(e - s - 1) for (s, e) in segs], default=0)
count = sum([ceil_div((e - s - 1), width) for (s, e) in segs]) if width > 0 else 0
print(count)
