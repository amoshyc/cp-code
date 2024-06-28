#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ll n; scanf("%lld", &n);
    double di = (sqrt(1 + 8 * n) - 1.0) / 2.0;
    ll i = ceil(di) - 1;

    // printf("i = %lld\n", i);
    ll s = i * (i + 1) / 2;
    // printf("s = %lld\n", s);
    ll delta = n - s;
    printf("%lld\n", delta);

    return 0;
}
