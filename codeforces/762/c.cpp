#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))

const int MAX_LEN = 100000;
const int INF = 0x3f3f3f3f;
int N, M;
string a, b;
int pref[MAX_LEN]; // pref[i] = j <-> min j $ b[0, i] is subsequence of a[0, j]
int suff[MAX_LEN]; // suff[i] = j <-> max j $ b[i, M-1] is subsequence of a[j, N-1]

void init() {
    fill(pref, pref + M, -1);
    fill(suff, suff + M, -1);

    int idx = 0;
    for (int i = 0; i < M; i++) {
        while (idx < N && a[idx] != b[i]) {
            idx++;
        }
        if (idx == N) break;
        pref[i] = idx;
        idx++;
    }

    idx = N - 1;
    for (int i = M - 1; i >= 0; i--) {
        while (idx >= 0 && a[idx] != b[i]) {
            idx--;
        }
        if (idx < 0) break;
        suff[i] = idx;
        idx--;
    }

    // cout << a << endl;
    // cout << b << endl;
    // for (int i = 0; i < M; i++) cout << pref[i] << " ";
    // cout << endl;
    // for (int i = 0; i < M; i++) cout << suff[i] << " ";
    // cout << endl;
}

bool C(int s, int t) { // [s, t] is removed
    if (s == 0 && t == M - 1) return true;
    if (s == 0) return suff[t + 1] != -1;
    if (t == M - 1) return pref[s - 1] != -1;
    return pref[s - 1] != -1 && suff[t + 1] != -1 && pref[s - 1] < suff[t + 1];
}

void solve() {
    if (pref[M - 1] != -1) { // no removal
        cout << b << endl;
        return;
    }

    auto search_tail = [&](int s) -> int {
        // binary search on t
        // 0 0 1 1 1
        int lb = s, ub = M - 1;
        if (C(s, lb)) return lb;
        if (!C(s, ub)) return -1;
        while (ub - lb > 1) {
            int m = (lb + ub) / 2;
            if (C(s, m)) ub = m;
            else lb = m;
        }
        return ub;
    };

    int mn_len = INF;
    int ss, tt;
    for (int s = 0; s < M; s++) { // [s, t]
        int t = search_tail(s);
        if (t != -1 && (t - s + 1) < mn_len) {
            mn_len = (t - s + 1);
            ss = s;
            tt = t;
        }
    }

    // cout << "ss: " << ss << endl;
    // cout << "tt: " << tt << endl;
    string res = b.substr(0, ss) + b.substr(tt + 1);
    if (sz(res) == 0)
        cout << "-" << endl;
    else
        cout << res << endl;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> a >> b;
    N = sz(a);
    M = sz(b);

    init();
    solve();

    return 0;
}
