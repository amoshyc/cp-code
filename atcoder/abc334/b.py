# a // b is the same as floor(a / b)
def floor_div(a, b):
    return a // b


# For ceil(a / b), we simply add 1 to a // b if needed
def ceil_div(a, b):
    return a // b + (1 if a % b != 0 else 0)


a, m, l, r = map(int, input().split())

# a + km >= l
# k >= ceil((l - a) / m)
lb = ceil_div(l - a, m)

# a + km <= r
# k <= floor((r - a) / m)
ub = floor_div(r - a, m)

print(max(0, ub - lb + 1))
