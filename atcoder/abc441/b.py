N, M = map(int, input().split())
S = set(input())
T = set(input())
Q = int(input())

ans = []
for _ in range(Q):
    w = input()

    all_in_s = all((c in S) for c in w)
    any_not_in_s = any((c not in S) for c in w)
    all_in_t = all((c in T) for c in w)
    any_not_in_t = any((c not in T) for c in w)

    if all_in_s and any_not_in_t:
        ans.append("Takahashi")
    elif all_in_t and any_not_in_s:
        ans.append("Aoki")
    else:
        ans.append("Unknown")

print("\n".join(ans))
