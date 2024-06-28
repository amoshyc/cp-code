#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 5000;
const ll INF = 1e18;

int N;
ll A[MAX_N];

ll solve(int l, int r) { // [l, r)
    ll vert = r - l;

    ll h = *min_element(A + l, A + r);
    for (int i = l; i < r; i++)
        A[i] -= h;

    ll hori = h;
    int s = l, t;
    for (;;) {
        while (s < r && A[s] == 0) s++;
        if (s >= r) break;
        t = s;
        while (t < r && A[t] != 0) t++;
        hori += solve(s, t);
        s = t;
    }

    return min(vert, hori);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N;
    for (int i = 0; i < N; i++)
        cin >> A[i];

    cout << solve(0, N) << endl;

    return 0;
}
