#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

struct P {
    ll m, f;
};

int N, D;
vector<P> a;

int main() {
    scanf("%d %d", &N, &D);
    for (int i = 0; i < N; i++) {
        ll f, m; scanf("%lld %lld", &f, &m);
        a.push_back((P) {f, m});
    }

    sort(a.begin(), a.end(), [](const P& p1, const P& p2) {
        return p1.m < p2.m;
    });

    int s = 0, t = 0;
    ll ans = -1;
    ll fsum = 0;
    for (;;) {
        while (t < N && a[t].m - a[s].m < D) {
            fsum += a[t++].f;
        }
        ans = max(ans, fsum);

        if (t == N) break;

        fsum -= a[s++].f;
    }

    printf("%lld\n", ans);

    return 0;
}
