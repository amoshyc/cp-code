#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    ll N, K;
    cin >> N >> K;

    auto P = vector<ll>(N, -1);
    auto C = vector<ll>(N, -1);
    for (ll i = 0; i < N; i++) {
        cin >> P[i];
        P[i]--;
    }
    for (ll i = 0; i < N; i++) {
        cin >> C[i];
    }

    auto vis = vector<bool>(N, false);
    auto ans = vector<ll>(N, ll(-1e18));

    for (ll root = 0; root < N; root++) {
        if (vis[root]) {
            continue;
        }

        vis[root] = true;
        auto cycle = vector<ll>({root});
        auto costs = vector<ll>({C[root]});
        auto u = P[root];
        while (u != root) {
            cycle.push_back(u);
            costs.push_back(C[u]);
            vis[u] = true;
            u = P[u];
        }

        ll L = cycle.size();
        ll S = accumulate(costs.begin(), costs.end(), 0LL);

        for (ll i = 0; i < L; i++) {
            ll cnt = 0;
            for (ll step = 1; step <= min(L, K); step++) {
                cnt += costs[(i + step) % L];
                auto val1 = cnt;
                auto val2 = cnt + (K - step) / L * S;
                ans[cycle[i]] = max({ans[cycle[i]], val1, val2});
            }
        }
    }

    cout << *max_element(ans.begin(), ans.end()) << endl;


    return 0;
}