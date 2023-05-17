K, N = map(int, input().split())
A = list(map(int, input().split()))
diff = [A[i + 1] - A[i] for i in range(N - 1)]
diff.append(K - sum(diff))
ans = sum(diff) - max(diff)
print(ans)
