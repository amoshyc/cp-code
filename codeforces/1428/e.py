from heapq import heappush, heappop

def f(length, n_piece):
    width = length // n_piece
    extra = length - width * n_piece
    return width ** 2 * (n_piece - extra) + (width + 1) ** 2 * extra


def solve():
    N, K = map(int, input().split())
    A = [int(x) for x in input().split()]

    ans = sum(a ** 2 for a in A)
    heap = []
    for i, a in enumerate(A):
        decrease = f(a, 2) - f(a, 1)
        n_piece = 2
        heappush(heap, (decrease, a, n_piece))

    for _ in range(K - N):
        decrease, a, n_piece = heappop(heap)
        ans += decrease
        
        decrease = f(a, n_piece + 1) - f(a, n_piece)
        n_piece = n_piece + 1
        heappush(heap, (decrease, a, n_piece))

    return ans

print(solve())

# import matplotlib.pyplot as plt
# fig, ax = plt.subplots()
# xs = list(range(1, 30))
# ys = [f(25, x) for x in xs]
# ax.plot(xs, ys, '.-')
# ax.set_xlabel('#piece')
# ax.set_ylabel('f(25, x)')
# plt.show()
