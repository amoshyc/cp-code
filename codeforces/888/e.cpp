#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
int getint() { int inp; scanf(" %d", &inp); return inp; }
ll getll() { ll inp; scanf(" %lld", &inp); return inp; }

int N, M;
vector<ll> A;

void pr(vector<ll>& v) {
    for (auto x : v) {
        printf("%lld ", x);
    }
    puts("");
}

void dfs(vector<ll>& v, int idx, int num) {
    for (int i = 0; i < (1 << num); i++) {
        ll sum = 0;
        for (int j = 0; j < num; j++) {
            if (i & (1 << j)) {
                sum = (sum + A[idx + j]) % M;
            }
        }
        v.push_back(sum);
    }
}

ll solve() {
    vector<ll> v1, v2;
    dfs(v1, 0, N / 2);
    dfs(v2, N / 2, N - N / 2);

    sort(v2.begin(), v2.end());

    // pr(v1);
    // pr(v2);

    int mx = -1;
    for (int x1 : v1) {
        auto lb = lower_bound(v2.begin(), v2.end(), M - 1 - x1);
        if (*lb != M - 1 - x1) {
            if (lb == v2.begin()) {
                continue;
            }
            lb--;
        }
        int x2 = *lb;
        mx = max(mx, x1 + x2);
    }

    return mx;
}

int main() {
    N = getint();
    M = getint();
    A = vector<ll>(N, 0);
    for (int i = 0; i < N; i++) {
        A[i] = getll() % M;
    }
    printf("%lld\n", solve());
    return 0;
}