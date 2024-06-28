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
    PolyHash(int N, ll base = 26, ll mod = 1e9 + 7) {
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

    vector<ll> transform(const vector<int> &v) {
        vector<ll> h(v.size());
        h[0] = v[0] % mod;
        for (size_t i = 1; i < v.size(); i++) {
            h[i] = (h[i - 1] + v[i] * powr[i] % mod) % mod;
        }
        return h;
    }

    ll substr(const vector<ll> &h, int l, int r) { // [l, r)
        if (l == 0) {
            return h[r - 1];
        } else {
            return ((h[r - 1] - h[l - 1] + mod) % mod) * pinv[l] % mod;
        }
    }
};

vector<int> alpha2int(const string &s) {
    vector<int> v(s.length());
    for (size_t i = 0; i < s.length(); i++) {
        v[i] = s[i] - 'a';
    }
    return v;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string S;
    cin >> S;
    int N = S.size();

    PolyHash tool(N, 26, 1e9 + 7);
    auto h = tool.transform(alpha2int(S));

    vector<pair<int, ll>> cands;
    for (int len = 1; len < N; len++) {
        ll prefix = tool.substr(h, 0, len);
        ll suffix = tool.substr(h, N - len, N);
        if (prefix == suffix) {
            cands.push_back({len, prefix});
        }
    }

    if (cands.size() == 0) {
        cout << "Just a legend" << endl;
        return 0;
    }

    auto check = [&](int m) -> bool {
        auto [len, val] = cands[m];
        for (int i = 1; i + len < N; i++) {
            if (tool.substr(h, i, i + len) == val) {
                return true;
            }
        }
        return false;
    };

    // check(m) = whether cands[m] exists in the middle of the string
    // 1 1 1 0 0 0
    int lb = 0, ub = cands.size() - 1;
    if (check(lb) == 0) {
        cout << "Just a legend" << endl;
        return 0;
    }
    if (check(ub) == 1) {
        auto [len, _] = cands[ub];
        cout << S.substr(0, len) << endl;
        return 0;
    }

    while (ub - lb > 1) {
        int m = (lb + ub) / 2;
        if (check(m)) {
            lb = m;
        } else {
            ub = m;
        }
    }

    auto [len, _] = cands[lb];
    cout << S.substr(0, len) << endl;

    return 0;
}