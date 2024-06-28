#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll mod = 1e6 + 3; // a prime

ll N, K;

ll mod_pow(ll a, ll b) {
    ll ans = 1;
    ll base = a;
    b %= (mod - 1);
    while (b) {
        if (b & 1)
            ans = ans * base % mod;
        base = base * base % mod;
        b >>= 1;
    }
    return ans;
}

ll mod_inv(ll a) {
    return mod_pow(a, mod - 2);
}

int main() {
    scanf("%lld %lld", &N, &K);

    if (N <= 62 && K > (1ll << N)) {
        puts("1 1");
        return 0;
    }

    // numerator
    ll nmr = 1;
    if (K - 1 >= mod) nmr = 0;
    else {
        ll val = mod_pow(2, N);
        for (int i = 1; i <= K - 1; i++) {
            ll item = (val - i + mod) % mod;
            (nmr *= item) %= mod;
        }
    }

    // denominator
    ll a = N % (mod - 1);
    ll b = (K - 1) % (mod - 1);
    ll dnm = mod_pow(2, a * b);

    // gcd
    ll v2 = (K - 1) - __builtin_popcountll(K - 1);
    ll inv = mod_inv(mod_pow(2, v2));

    // printf("%lld, %lld, %lld, %lld\n", nmr, dnm, inv, v2);

    (nmr *= inv) %= mod;
    (dnm *= inv) %= mod;
    (nmr = dnm - nmr + mod) %= mod;

    printf("%lld %lld\n", nmr, dnm);

    return 0;
}
