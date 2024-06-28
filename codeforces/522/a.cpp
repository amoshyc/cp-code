#include <bits/stdc++.h>
using namespace std;

const int MAX_V = 400;
int E;
vector<int> g[MAX_V];

map<string, int> id;
int get_id(string s) {
    for (char& c : s) {
        c = tolower(c);
    }

    if (id.find(s) == id.end()) {
        id[s] = id.size();
    }

    return id[s];
}

bool vis[MAX_V];
int max_dep = -1;
void dfs(int u, int dep) {
    vis[u] = true;

    if (g[u].size() == 0) {
        max_dep = max(max_dep, dep);
        return;
    }

    for (int v : g[u]) {
        if (!vis[v]) {
            dfs(v, dep + 1);
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> E;
    string a, b, rep;
    while (E--) {
        cin >> a >> rep >> b;
        int id_a = get_id(a);
        int id_b = get_id(b);
        g[id_b].push_back(id_a);
    }

    dfs(get_id("Polycarp"), 1);

    cout << max_dep << "\n";

    return 0;
}
