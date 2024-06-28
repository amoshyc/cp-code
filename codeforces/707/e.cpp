#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 2000;
const int MAX_M = 2000;
const int MAX_K = 2000;

struct Query {
    int r1, r2, c1, c2; ll ans;
    bitset<MAX_K> is_on;
};

struct Point {
    int r, c; ll w;
};

int N, M, K, Q;
vector<Query> queries;
bitset<MAX_K> is_on;
vector<Point> garland[MAX_K];

ll bit[MAX_N + 1][MAX_M + 1]; // 1-based index
ll sum(int a, int b) {
    ll s = 0;
    for (int i = a; i > 0; i -= (i & -i))
        for (int j = b; j > 0; j -= (j & -j))
            s += bit[i][j];
    return s;
}
void add(int a, int b, ll x) {
    for (int i = a; i <= N; i += (i & -i))
        for (int j = b; j <= M; j += (j & -j))
            bit[i][j] += x;
}

int main() {
    scanf("%d %d %d", &N, &M, &K);

    is_on.flip(); // every garland is on inititally

    for (int i = 0; i < K; i++) {
        int len; scanf("%d", &len);
        for (int j = 0; j < len; j++) {
            Point p;
            scanf("%d %d %lld", &p.r, &p.c, &p.w);
            garland[i].push_back(p);
        }
    }

    scanf("%d", &Q);
    while (Q--) {
        char cmd[10]; scanf("%s", cmd);

        if (cmd[0] == 'S') {
            int id; scanf("%d", &id); id--;
            is_on.flip(id);
        }
        else {
            Query q;
            scanf("%d %d %d %d", &q.r1, &q.c1, &q.r2, &q.c2);
            q.ans = 0;
            q.is_on = is_on;
            queries.push_back(q);
        }
    }

    for (int i = 0; i < K; i++) {
        auto &v = garland[i];

        for (auto p : v) {
            add(p.r, p.c, +p.w);
        }

        // for (int r = 1; r <= N; r++) {
        //     for (int c = 1; c <= M; c++) {
        //         ll x = sum(r, c) - sum(r, c - 1) - sum(r - 1, c) + sum(r - 1, c - 1);
        //         printf("%lld ", x);
        //     }
        //     puts("");
        // }

        for (auto &q : queries) {
            if (q.is_on[i]) {
                ll cnt = 0;
                cnt += sum(q.r2, q.c2);
                cnt -= sum(q.r1 - 1, q.c2);
                cnt -= sum(q.r2, q.c1 - 1);
                cnt += sum(q.r1 - 1, q.c1 - 1);
                q.ans += cnt;
            }
        }

        for (auto p : v) {
            add(p.r, p.c, -p.w);
        }
    }


    for (auto q : queries) {
        printf("%lld\n", q.ans);
    }

    return 0;
}
