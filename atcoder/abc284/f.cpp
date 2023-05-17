#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

ll powmod(ll a, ll b, ll m) {
    ll base = a;
    ll res = 1;
    while (b) {
        if (b & 1) {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    return res;
}

struct PolyHash {
    ll mod;
    ll base;
    vector<ll> powr;
    vector<ll> pinv;

    // N is max length; mod should be prime
    PolyHash(int N, ll base = 31, ll mod = 1e9 + 7) {
        this->base = base;
        this->mod = mod;
        powr = vector<ll>(N, 1);
        pinv = vector<ll>(N, 1);
        for (int i = 1; i < N; i++) {
            powr[i] = powr[i - 1] * base % mod;
        }
        pinv[N - 1] = powmod(powr[N - 1], mod - 2, mod);
        for (int i = N - 2; i >= 0; i--) {
            pinv[i] = pinv[i + 1] * base % mod;
        }
    }

    vector<ll> hash(const vector<int> &v) {
        for (auto &&x : v) {
            assert(x != 0);
        }

        vector<ll> h(v.size());
        h[0] = v[0] % mod;
        for (size_t i = 1; i < v.size(); i++) {
            h[i] = (h[i - 1] + v[i] * powr[i] % mod) % mod;
        }
        return h;
    }

    ll range(const vector<ll> &h, int l, int r) { // [l, r)
        if (l == r) {
            return 0;
        } else if (l == 0) {
            return h[r - 1];
        } else {
            return ((h[r - 1] - h[l - 1] + mod) % mod) * pinv[l] % mod;
        }
    }
};

vector<int> string2vector(const string &s) {
    vector<int> v(s.length());
    for (size_t i = 0; i < s.length(); i++) {
        if (s[i] >= 'a' && s[i] <= 'z') {
            v[i] = s[i] - 'a' + 1;
        } else {
            v[i] = s[i] - 'A' + 1;
        }
    }
    return v;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    string t;
    cin >> t;

    auto a = t;
    auto b = t;
    reverse(b.begin(), b.end());

    auto h = PolyHash(2 * n);
    auto ha = h.hash(string2vector(a));
    auto hb = h.hash(string2vector(b));

    for (int i = 0; i <= n; i++) {
        // S = rev(T[i..(i + n)])
        //   => S = revT[(n - i)..(2n - i)]
        // S[0..i] = T[0..i]
        //   => revT[(n - i)..n] = T[0..i]
        // S[i..n] = T[(n + i)..(2n)]
        //   => revT[n..(2n - i)] = T[(n + i)..(2n)]
        bool ok1 = h.range(hb, n - i, n) == h.range(ha, 0, i);
        bool ok2 = h.range(hb, n, 2 * n - i) == h.range(ha, n + i, 2 * n);
        if (ok1 && ok2) {
            auto s = t.substr(i, n);
            reverse(s.begin(), s.end());
            cout << s << endl;
            cout << i << endl;
            exit(0);
        }
    }

    cout << -1 << endl;

    return 0;
}