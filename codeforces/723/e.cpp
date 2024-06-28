#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 200;
const int MAX_M = MAX_N * (MAX_N + 1) / 2;

// list
struct Node {
    int val;
    Node *nxt;
};
int NN = 0;
Node pool[MAX_M + 1]; // 給定的圖 E 條有向邊，環最大有 E + 1 個節點
Node* root = NULL;

Node* new_node(int v) {
    Node* it = &pool[NN++];
    it->val = v;
    it->nxt = NULL;
    return it;
}

// graph
int N, M; // V, E
vector<int> cycle; // 存找到的（小）歐拉環
multiset<int> g[MAX_N]; // 用 multiset 存

void dfs(int u) {
    cycle.push_back(u);
    if (g[u].size() > 0) {
        int v = *g[u].begin();
        g[u].erase(g[u].begin()); // remove 1 edge
        g[v].erase(g[v].find(u)); // remove 1 edge
        dfs(v);
    }
}

void find_euler_cycle(int s) {
    root = new_node(s);
    for (Node* it = root; it != NULL; it = it->nxt) {
        if (g[it->val].size() > 0) {
            cycle.clear();
            dfs(it->val);

            Node* nxt = it->nxt;
            Node* pre = it;
            for (int i = 1; i < int(cycle.size()); i++) {
                Node* temp = new_node(cycle[i]);
                pre->nxt = temp;
                pre = temp;
            }
            pre->nxt = nxt;
        }
    }
}

typedef pair<int, int> pii;
int deg[MAX_N];

int main() {
    int TC;
    scanf("%d", &TC);
    while (TC--) {
        scanf("%d %d", &N, &M);

        NN = 0;
        fill(deg, deg + N, 0);
        for (int u = 0; u < N; u++)
            g[u].clear();

        for (int i = 0; i < M; i++) {
            int u, v; scanf("%d %d", &u, &v); u--; v--;
            g[u].insert(v);
            g[v].insert(u);
            deg[u]++;
            deg[v]++;
        }

        vector<int> odds;
        for (int u = 0; u < N; u++) {
            if (deg[u] % 2 == 1) {
                odds.push_back(u);
            }
        }

        set<pii> added; // added edges
        for (int i = 0; i < int(odds.size()); i += 2) {
            int u = odds[i], v = odds[i + 1];
            g[u].insert(v);
            g[v].insert(u);
            added.insert(minmax(u, v));
        }

        printf("%d\n", N - int(odds.size()));

        for (int u = 0; u < N; u++) { // 如果圖是多個 disjoint 單元
            if (g[u].size() > 0) {
                find_euler_cycle(u);

                for (Node* it = root; it->nxt != NULL; it = it->nxt) {
                    int u = it->val, v = it->nxt->val;

                    auto pos = added.find(minmax(u, v));
                    if (pos == added.end()) {
                        printf("%d %d\n", u + 1, v + 1);
                    }
                    else {
                        added.erase(pos);
                    }
                }
            }
        }
    }

    return 0;
}
