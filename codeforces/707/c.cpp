#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ll a;
    scanf("%lld", &a);

    if (a <= 2) {
        puts("-1");
        return 0;
    }

    if (a & 1) {
        // a = m^2 - n^2 (只求一組解)
        // 1 * a = (m + n)(m - n)
        // n = (a - 1) / 2
        // m = n + 1
        ll n = (a - 1) / 2;
        ll m = n + 1;

        ll b = 2 * m * n;
        ll c = m * m + n * n;
        printf("%lld %lld\n", b, c);
    }
    else {
        // a = 2 * m * n (只求一組解，取 n = 1)
        // n = 1
        // m = a / 2
        ll n = 1;
        ll m = a / 2;

        ll b = m * m - n * n;
        ll c = m * m + n * n;
        printf("%lld %lld\n", b, c);
    }

    return 0;
}
