import sys
import numpy as np
from scipy.optimize import linear_sum_assignment

np.random.seed(42)

N, M, K, R = map(int, input().split())
D = np.array([[int(x) for x in input().split()] for _ in range(N)], dtype=int)
D = D.sum(axis=1)

sp = np.abs(np.random.normal(size=(M, K)))
norm = np.sqrt(np.sum(sp ** 2, axis=1, keepdims=True))
Q = np.random.uniform(20.0, 60.0, size=(M, K)) / norm
S = np.round(Q * sp).astype(float).sum(axis=1)

G = [[] for _ in range(N)]
indeg = [0 for _ in range(N)]
for _ in range(R):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1
    G[u].append(v)
    indeg[v] += 1
indeg = np.int32(indeg)

start = np.zeros(M, dtype=int) - 1
assign = np.zeros(M, dtype=int) - 1
status = np.zeros(N, dtype=int) - 1

for day in range(2000):
    if day % 1 == 0:
        (task_ids,) = np.nonzero((status == -1) & (indeg == 0))
        (person_ids,) = np.nonzero(assign == -1)

        # print(day, 'tids:', task_ids, 'pids:', person_ids, file=sys.stderr)

        if len(task_ids) > 0 and len(person_ids) > 0:
            T = D[task_ids].reshape(-1, 1) - S[person_ids].reshape(1, -1)
            rr, cc = linear_sum_assignment(T)
            start[person_ids[cc]] = day
            assign[person_ids[cc]] = task_ids[rr]
            status[task_ids[rr]] = 0

            today = [
                '{} {}'.format(pid + 1, tid + 1)
                for pid, tid in zip(person_ids[cc], task_ids[rr])
            ]
            print(len(today), ' '.join(today), flush=True)
        else:
            print(0, flush=True)
    else:
        print(0, flush=True)
    
    n, *fs = [int(x) for x in input().split()]
    if n == -1:
        break
    if n > 0:
        person_ids = np.int32(fs) - 1
        task_ids = assign[person_ids]

        t = day - start[person_ids] + 1
        d = D[task_ids]
        s = d - t
        S[person_ids] = S[person_ids] * 0.5 + s

        start[person_ids] = -1
        assign[person_ids] = -1
        status[task_ids] = 1

        for task_id in task_ids:
            for v in G[task_id]:
                indeg[v] -= 1

# print(S, file=sys.stderr)