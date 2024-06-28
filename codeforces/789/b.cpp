#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
#define sz(x) (int(x.size()))

const int MAX_M = 100000;
ll b, q, l, m;
unordered_set<ll> a;

bool in_a(ll x) {
    return a.find(x) != a.end();
}

bool in_l(ll x) {
    return abs(x) <= l;
}

bool legal(ll x) {
    return !in_a(x) && in_l(x);
}

int main() {
    scanf("%lld %lld %lld %lld", &b, &q, &l, &m);
    for (int i = 0; i < m; i++) {
        ll inp; scanf("%lld", &inp);
        a.insert(inp);
    }

    if (b == 0) {
        if (legal(b)) {
            puts("inf");
        }
        else {
            puts("0");
        }
        return 0;
    }

    if (q == 0) {
        // b, 0, 0, 0, ...
        if (legal(b) && legal(0)) {
            puts("inf");
        }
        else if (legal(b) && !legal(0)) {
            puts("1");
        }
        else if (in_l(b) && legal(0)) {
            puts("inf");
        }
        else {
            puts("0");
        }

        return 0;
    }

    if (q == +1) {
        // b, b, b, b, ...
        if (legal(b)) {
            puts("inf");
        }
        else {
            puts("0");
        }
        return 0;
    }

    if (q == -1) {
        // b, -b, b, -b, ...
        if (legal(b) && legal(-b)) {
            puts("inf");
        }
        else if (legal(b) && in_l(-b)) {
            puts("inf");
        }
        else if (legal(b) && !legal(-b)) {
            puts("1");
        }
        else if (in_l(b) && legal(-b)) {
            puts("inf");
        }
        else {
            puts("0");
        }
        return 0;
    }

    int cnt = 0;
    ll x = b;
    if (legal(b)) {
        cnt++;
    }

    for (;;) {
        if (abs(x * q) > l) {
            break;
        }
        if (!in_a(x * q)) {
            cnt++;
        }
        x = x * q;
    }

    printf("%d\n", cnt);

    return 0;
}
