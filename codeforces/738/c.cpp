#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int INF = 0x3f3f3f3f;

const int MAX_N = 200000;
const int MAX_K = 200000;
int N, K, S, T;

struct Car {
    int price, cap;
};

int mx_cap = -1;
Car cars[MAX_N];
int pos[MAX_K + 1];

bool C(int cap) {
    int total = 0;
    int cur = 0;
    for (int i = 0; i < K; i++) {
        int need = pos[i] - cur;
        // assume z km run in acc mode
        // fuel use: z * 2 + (need - z) * 1
        // time use: z + (need - z) * 2
        // fuel use <= cap
        // z <= cap - need
        int z = cap - need;
        if (z > need) z = need;
        if (z < 0) return false; // z should >= 0
        if (z * 2 + (need - z) * 1 > cap) return false; // check again
        total += z + (need - z) * 2;
        cur = pos[i];
    }
    return (total <= T);
}

int solve() {
    sort(pos, pos + K);

    // binary search on cap
    // 0 0 0 1 1 1
    // C(x) = Can a car of cap = x run S km in T min
    int lb = 0, ub = mx_cap;
    // if (C(lb)) impossible
    if (!C(ub)) return -1;
    while (ub - lb > 1) {
        int m = (lb + ub) / 2;
        if (C(m)) ub = m;
        else lb = m;
    }

    int mn_price = INF;
    for (int i = 0; i < N; i++) {
        if (cars[i].cap >= ub) {
            mn_price = min(mn_price, cars[i].price);
        }
    }

    return mn_price;
}

int main() {
    scanf("%d %d %d %d", &N, &K, &S, &T);
    for (int i = 0; i < N; i++) {
        scanf("%d %d", &cars[i].price, &cars[i].cap);
        mx_cap = max(mx_cap, cars[i].cap);
    }
    for (int i = 0; i < K; i++) {
        scanf("%d", &pos[i]);
    }
    pos[K++] = S;

    printf("%d\n", solve());

    return 0;
}
