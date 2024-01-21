#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;

using ll = long long;
using City = tuple<ll, ll, ll>;

ll dist(City city1, City city2) {
    auto [a, b, c] = city1;
    auto [p, q, r] = city2;
    return abs(p - a) + abs(q - b) + max(0ll, r - c);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    vector<City> cities;
    for (int i = 0; i < N; i++) {
        ll x, y, z;
        cin >> x >> y >> z;
        cities.push_back({x, y, z});
    }

    const ll inf = 1e15;
    auto dp = vector<vector<ll>>(1 << N, vector<ll>(N, inf));

    dp[1 << 0][0] = 0;

    for (int s = 1; s < (1 << N); s++) {
        for (int v = 0; v < N; v++) {
            if (s & (1 << v)) {
                for (int u = 0; u < N; u++) {
                    if ((s & (1 << u))) {
                        dp[s][v] =
                            min(dp[s][v], dp[s ^ (1 << v)][u] +
                                              dist(cities[u], cities[v]));
                    }
                }
            }
        }
    }

    ll ans = inf;
    for (int v = 0; v < N; v++) {
        ans = min(ans, dp[(1 << N) - 1][v] + dist(cities[v], cities[0]));
    }
    cout << ans << endl;

    return 0;
}