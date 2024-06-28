#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 2000;
int N;
vector<int> child[MAX_N];
vector<int> root;
int max_dep = -1;

void dfs(int u, int dep) {
    if (child[u].size() == 0) {
        max_dep = max(max_dep, dep);
    }

    for (int v : child[u]) {
        dfs(v, dep + 1);
    }
}

int main() {
    scanf("%d", &N);

    for (int v = 0; v < N; v++) {
        int p; scanf("%d", &p);
        if (p != -1) {
            p--;
            child[p].push_back(v);
        }
        else {
            root.push_back(v);
        }
    }

    for (int r : root) {
        dfs(r, 1);
    }

    printf("%d\n", max_dep);

    return 0;
}
