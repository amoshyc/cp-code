// #include <bits/stdc++.h>
#include <iostream>
#include <vector>
#include <functional>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int n, m;
    cin >> n >> m;

    vector<vector<int>> G(n);
    for (int i = 0; i < m; i++) {
        int u, v;
        cin >> u >> v;
        u--, v--;
        G[u].push_back(v);
        G[v].push_back(u);
    }
    vector<bool> vis(n);
    
    long long res = 0, cnt = 0, s_cnt = 0;
    vector<int> b;
    vector<int> col(n, -1);
    bool ok;
    function<void (int, int)> DFS = [&](int u, int c) {
        col[u] = c;
        cnt++;
        if (c == 0) {
            b.push_back(u);
        }
        for (auto &v : G[u]) {
            if (col[v] == -1) {
                DFS(v, c ^ 1);
            } else if (col[v] == c) {
                ok = false;
                return ;
            }
        }
        return ;
    };

    int c_cnt = 0;
    for (int i = 0; i < n; i++) {
        if (col[i] == -1) {
            cnt = 0;
            b.clear();
            ok = true;
            DFS(i, 0);
            if (ok) {
                c_cnt++;
                res += (n - cnt) * cnt;
                for (auto &v : b) {
                    s_cnt += cnt - (int) b.size() - (int) G[v].size();
                }
            } else {
                res = s_cnt = 0;
                break;
            }
        }
    }

    long long ans = 0;
    if (c_cnt != 1) {
        ans += res / 2;
    }
    ans += s_cnt;

    cout << ans << '\n';

    return 0;
}