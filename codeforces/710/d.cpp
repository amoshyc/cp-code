#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

struct Item {
    ll m, r;
};

ll extgcd(ll a, ll b, ll& x, ll&y) {
    if (b == 0) {
        x = 1;
        y = 0;
        return a;
    }
    else {
        ll d = extgcd(b, a % b, y, x);
        y -= (a / b) * x;
        return d;
    }
}

Item extcrt(const vector<Item>& v) {
    ll m1 = v[0].m, r1 = v[0].r, x, y;

    for (int i = 1; i < int(v.size()); i++) {
        ll m2 = v[i].m, r2 = v[i].r;
        ll g = extgcd(m1, m2, x, y); // now x = (m/g)^(-1)

        if ((r2 - r1) % g != 0)
            return {-1, -1};

        ll k = (r2 - r1) / g * x % (m2 / g);
        k = (k + m2 / g) % (m2 / g); // for the case k is negative

        ll m = m1 * m2 / g;
        ll r = (m1 * k + r1) % m;

        m1 = m;
        r1 = (r + m) % m; // for the case r is negative
    }

    return (Item) {m1, r1};
}

int main() {
    ll a1, b1, a2, b2, L, R;
    scanf("%lld %lld %lld %lld %lld %lld", &a1, &b1, &a2, &b2, &L, &R);
    vector<Item> item;
    item.push_back((Item) {a1, (b1 % a1 + a1) % a1});
    item.push_back((Item) {a2, (b2 % a2 + a2) % a2});
    Item res = extcrt(item);

    if (res.m == -1) {
        puts("0");
        return 0;
    }

    L = max(L, max(b1, b2));
    ll s = (L - res.r) / res.m * res.m + res.r;
    ll t = (R - res.r) / res.m * res.m + res.r;

    if (s < L)
        s += res.m;
    if (t > R) // in case that R and t are nagative
        t -= res.m;

    // if (s > R || t < L) {
    if (s > t) {
        puts("0");
        return 0;
    }

    ll ans = (t - s) / res.m + 1;
    printf("%lld\n", ans);

    return 0;
}
