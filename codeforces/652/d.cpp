#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 200000;

struct S {
    int id, l, r;
    bool operator < (const S& s) const {
        return r < s.r;
    }
};

int N;
S segs[MAX_N];
int ans[MAX_N];
int bit[2 * MAX_N + 1];

int sum(int i) {
    int s = 0;
    while (i > 0) {
        s += bit[i];
        i -= (i & (-i));
    }
    return s;
}

void add(int i, int x) {
    while (i <= 2 * N) {
        bit[i] += x;
        i += (i & (-i));
    }
}

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        segs[i].id = i;
        scanf("%d %d", &segs[i].l, &segs[i].r);
    }

    vector<int> es;
    for (int i = 0; i < N; i++) {
        es.push_back(segs[i].l);
        es.push_back(segs[i].r);
    }
    sort(es.begin(), es.end());

    for (int i = 0; i < N; i++) {
        auto it_l = lower_bound(es.begin(), es.end(), segs[i].l);
        auto it_r = lower_bound(es.begin(), es.end(), segs[i].r);
        segs[i].l = it_l - es.begin() + 1;
        segs[i].r = it_r - es.begin() + 1;
    }

    sort(segs, segs + N);
    for (int i = 0; i < N; i++) {
        ans[segs[i].id] = i - sum(segs[i].l - 1);
        add(segs[i].l, +1);
    }

    for (int i = 0; i < N; i++)
        printf("%d\n", ans[i]);

    return 0;
}
