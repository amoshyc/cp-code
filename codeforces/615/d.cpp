#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MOD = 1e9 + 7;
const int MAX_N = 200000;
int N;
ll p[MAX_N];
int NN;
ll fb[MAX_N]; // 底數（質數）
ll fe[MAX_N]; // 指數

ll fast_pow(ll a, ll b) {
    b %= (MOD - 1); // 費馬小定理
    ll ans = 1;
    ll base = a % MOD;
    while (b) {
        if (b & 1) ans = (ans * base) % MOD;
        base = (base * base) % MOD;
        b >>= 1;
    }
    return ans;
}

int main() {
    // ios::sync_with_stdio(false);
    // cin.tie(0);

    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%lld", &p[i]);
    }

    // merge same base
    NN = 0;
    sort(p, p + N);

    fb[NN] = p[0];
    fe[NN] = 1;
    NN++;
    for (int i = 1; i < N; i++) {
        if (p[i] == p[i - 1]) {
            fe[NN - 1]++;
        }
        else {
            fb[NN] = p[i];
            fe[NN] = 1;
            NN++;
        }
    }

    ll ans = 1;
    ll cnt = 1;
    for (int i = 0; i < NN; i++) {
        ll b = fast_pow(fb[i], fe[i] * (fe[i] + 1) / 2);
        ans = fast_pow(ans, fe[i] + 1) * fast_pow(b, cnt) % MOD;
        cnt = cnt * (fe[i] + 1) % (MOD - 1);
    }

    // (p1^0 * p1^1 * ... * p1^e1)^d1 * (p2^0 * p2^1 * ... * p2^e2)^d2 * ...
    // b[i] = (pi^0 * pi^1 * ... * pi^ei)^di
    // ans = 複製 (fe[i] + 1) 份之前所有組合 * 新乘入的數字積
    // cnt[i] = (d1 + 1) * (d2 + 1) * ... * (d[i] + 1) 之前有幾種組合

    printf("%lld\n", ans);

    return 0;
}
