N, M = map(int, input().split())
A = [int(x) for x in input().split()]


def ok(x):
    # if using subsidy x, will the total subsidy be less or equal to M?
    return sum(min(a, x) for a in A) <= M


# ok(x) function forms a ... 1 1 1 0 0 0 ... distribution over all possible subsidy x
# We want to find the last x that has ok(x) = 1

lb = 0  # minimum possible x
ub = max(A)  # maximum possible x

# Special case 1:
# ok(x) is 0 for all x
# that is, a 0 0 0 0 0 0 distribution.
# that is, ok(lb) = 0
# It won't happen in this problem.

# Special case 2:
# ok(x) is 1 for all x
# that is, a 1 1 1 1 1 1 distribution
# that is, ok(ub) = 1
if ok(ub):
    print("infinite")
    exit()

# General case:
# binary search to find the partition point
# lb will converge to the largest x that ok(x) = 1
# ub will converge to the smallest x that ok(x) = 0
while ub - lb > 1:
    m = (lb + ub) // 2
    if ok(m):
        lb = m
    else:
        ub = m

print(lb)
