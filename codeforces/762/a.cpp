#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
#define sz(x) (int(x.size()))

int main() {
    ll n, k;
    scanf("%lld %lld", &n, &k);

    vector<ll> divs;
    for (ll i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            divs.push_back(i);
            if (i * i != n) {
                divs.push_back(n / i);
            }
        }
    }

    sort(divs.begin(), divs.end());

    if (k > sz(divs)) {
        puts("-1");
        return 0;
    }
    printf("%lld\n", divs[k - 1]);

    return 0;
}
