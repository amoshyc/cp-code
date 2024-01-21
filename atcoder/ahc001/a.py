import numpy as np

np.random.seed(42)


def box_area(boxes):
    return (boxes[:, 2] - boxes[:, 0] + 1) * (boxes[:, 3] - boxes[:, 1] + 1)


def has_overlap(boxes, idx):
    N = len(boxes)
    boxesA = np.broadcast_to(boxes[idx].reshape(1, 4), (N, 4))
    boxesB = boxes
    dx = np.minimum(boxesA[:, 2], boxesB[:, 2]) - np.maximum(boxesA[:, 0], boxesB[:, 0])
    dy = np.minimum(boxesA[:, 3], boxesB[:, 3]) - np.maximum(boxesA[:, 1], boxesB[:, 1])
    dx[idx] = 0
    dy[idx] = 0
    return np.any((dx > 0) & (dy > 0))


def solve(requests):
    N = len(requests)

    boxes = np.stack(
        [requests[:, 0], requests[:, 1], requests[:, 0] + 1, requests[:, 1] + 1], axis=1
    )

    for t in range(3000):
        area = box_area(boxes)
        prob = np.log(requests[:, 2]) - np.log(area)
        prob = np.clip(prob, 0, None)
        prob = prob / prob.sum()
        idx = np.random.choice(np.arange(N), p=prob)
        # idx = np.random.randint(0, N)

        if t <= 1000:
            l = 200
        elif t <= 1750:
            l = 100
        elif t <= 2500:
            l = 10
        else:
            l = 1
        increments = np.int32(
            [[0, 0, 0, +l], [0, -l, 0, 0], [-l, 0, 0, 0], [0, 0, +l, 0]]
        )
        increments = increments[np.random.permutation(4)]

        for inc in increments:
            boxes[idx] += inc
            if has_overlap(boxes, idx) == False:
                break
            boxes[idx] -= inc

        boxes[idx] = np.clip(boxes[idx], 0, 10000)

    return boxes


if __name__ == '__main__':
    N = int(input())
    requests = []
    for _ in range(N):
        x, y, r = map(int, input().split())
        requests.append((x, y, r))
    requests = np.int32(requests)

    boxes = solve(requests)
    for a, b, c, d in boxes:
        print(a, b, c, d)

    # import matplotlib as mpl
    # import matplotlib.pyplot as plt
    # from common import *
    # fig, ax = plt.subplots()
    # visualize(ax, requests, boxes)
    # ax.set_title(evaluate(requests, boxes))
    # plt.show()
