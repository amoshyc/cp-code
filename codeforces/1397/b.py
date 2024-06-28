import sys
import math

N = int(input())
A = [int(x) for x in input().split()]
A.sort()

def find_cost(c):
    cost = 0
    for i in range(N):
        cost += abs(A[i] - pow(c, i))
    return cost

pivot = A[-1] ** (1 / (N - 1))
lb = math.floor(pivot)
ub = math.ceil(pivot)
cost1 = find_cost(lb)
cost2 = find_cost(ub)
print(min(cost1, cost2))

