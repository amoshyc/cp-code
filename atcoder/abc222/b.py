N, P = map(int, input().split())
A = [int(x) for x in input().split()]
B = [x for x in A if x < P]
print(len(B))