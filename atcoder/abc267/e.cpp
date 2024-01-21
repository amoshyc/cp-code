#include <algorithm>
#include <iostream>
#include <vector>
#include <queue>
using namespace std;

using ll = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, M;
    cin >> N >> M;
    auto A = vector(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }
    auto adj = vector(N, vector<int>());
    for (int i = 0; i < M; i++) {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    auto check = [&](ll m) -> bool {
        auto C = vector(N, 0ll);
        for (int u = 0; u < N; u++) {
            for (int v : adj[u]) {
                C[u] += A[v];
            }
        }

        auto que = queue<int>();
        auto rem = vector<bool>(N, false);
        for (int u = 0; u < N; u++) {
            if (C[u] <= m) {
                que.push(u);
                rem[u] = true;
            }
        }

        while (que.size() > 0) {
            int u = que.front();
            que.pop();

            for (int v : adj[u]) {
                if (not rem[v]) {
                    C[v] -= A[u];
                    if (C[v] <= m) {
                        que.push(v);
                        rem[v] = true;
                    }
                }
            }
        }

        for (int u = 0; u < N; u++) {
            if (not rem[u]) {
                return false;
            }
        }
        return true;
    };

    ll lb = -1;
    ll ub = 1;
    for (int x: A) {
        ub += x;
    }
    while (ub - lb > 1) {
        ll m = (ub + lb) / 2;
        if (check(m)) {
            ub = m;
        } else {
            lb = m;
        }
    }

    cout << ub << endl;

    return 0;
}