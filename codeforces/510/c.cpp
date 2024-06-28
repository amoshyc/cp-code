#include <bits/stdc++.h>
using namespace std;

int N;
string s[100];
vector<int> g[26];
int indegree[26];
vector<int> ans;

void add_edge(char c1, char c2) {
    g[c1 - 'a'].push_back(c2 - 'a');
    indegree[c2 - 'a']++;

    // cout << c1 << " -> " << c2 << endl;
}

void topological_sort() {
    queue<int> q;
    for (int i = 0; i < 26; i++) {
        if (indegree[i] == 0) {
            q.push(i);
        }
    }

    ans.clear();
    while (!q.empty()) {
        int v = q.front(); q.pop();

        ans.push_back(v);

        for (int u : g[v]) {
            indegree[u]--;
            if (indegree[u] == 0) {
                q.push(u);
            }
        }
    }
}

bool solve() {
    for (int i = 1; i < N; i++) {
        int pos = -1;
        for (size_t j = 0; j < min(s[i - 1].length(), s[i].length()); j++) {
            if (s[i - 1][j] != s[i][j]) {
                pos = j;
                break;
            }
        }

        if (pos == -1) {
            if (s[i-1].length() > s[i].length())
                return false;
            continue;
        }

        add_edge(s[i - 1][pos], s[i][pos]);
    }

    topological_sort();

    if (ans.size() != 26)
        return false;

    return true;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N;
    for (int i = 0; i < N; i++)
        cin >> s[i];

    if (!solve()) cout << "Impossible\n";
    else {
        for (int v : ans) {
            cout << char(v + 'a');
        }
        cout << endl;
    }

    return 0;
}
