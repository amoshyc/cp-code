#include <bits/stdc++.h>
using namespace std;

const int max_n = 1000000;
int N;
int nxt[max_n + 1];
int ans[max_n + 1];
vector<int> path;
bool visited[max_n + 1];
vector<vector<int>> cnt[max_n + 1];

void print_path(const vector<int>& v) {
    for (const auto& u : v)
        printf("%d-> ", u);
}

int dfs(int v) {
    path.clear();

    visited[v] = true;
    path.push_back(v);

    int u = nxt[v];
    while (u != v) {
        visited[u] = true;
        path.push_back(u);
        u = nxt[u];
    }

    return path.size();
}

void merge_cycle(int len) {
    const vector<vector<int>>& cycles = cnt[len];
    const int size = cycles.size();

    for (int i = 0; i < size / 2; i++) {
        // merge cycles[i * 2 + 0], cycles[i * 2 + 1]
        vector<int> new_cycle;

        for (int j = 0; j < len; j++) {
            new_cycle.push_back(cycles[i * 2 + 0][j]);
            new_cycle.push_back(cycles[i * 2 + 1][j]);
        }

        // printf("merging len=%d\n", len);
        // print_path(cycles[i * 2 + 0]); puts("");
        // print_path(cycles[i * 2 + 1]); puts("");
        // printf("=> "); print_path(new_cycle); puts("");

        // fill the ans
        for (int j = 0; j < 2 * len; j++) {
            // new_cycle[j] -> new_cycle[j + 1]
            ans[new_cycle[j]] = new_cycle[(j + 1) % (2 * len)];
        }
    }
}

void reverse_cycle(int len) {
    const vector<vector<int>>& cycles = cnt[len];
    for (const vector<int> cycle : cycles) {
        vector<int> origin(len, 0);
        int idx = 0;
        for (int j = 0; j < len; j++) {
            origin[idx] = cycle[j];
            idx = (idx + 2) % len;
        }

        for (int j = 0; j < len; j++) {
            // printf("mapping %d to %d\n", origin[j], origin[(j+1) % len]);
            ans[origin[j]] = origin[(j+1) % len];
        }
    }
}

bool solve() {
    for (int i = 1; i <= N; i++) {
        if (!visited[i]) {
            int len = dfs(i);
            cnt[len].push_back(path);

            // printf("found path at %d(%d): ", i, len);
            // print_path(path);
            // puts("");
        }
    }

    // for (int i = 1; i <= N; i++) {
    //     printf("%d(%zu)\n", i, cnt[i].size());
    // }

    for (int i = 2; i <= N; i += 2) {
        if (cnt[i].size() % 2 == 1)
            return false;
        else
            merge_cycle(i);
    }

    for (int i = 1; i <= N; i += 2)
        reverse_cycle(i);

    return true;
}

int main() {
    scanf("%d", &N);
    for (int i = 1; i <= N; i++) {
        scanf("%d", &nxt[i]);
    }

    if (solve() == false) {
        puts("-1");
    }
    else {
        for (int i = 1; i <= N; i++)
            printf("%d ", ans[i]);
        puts("");
    }

    // vector<int> verify(N + 1, 0);
    // for (int i = 1; i <= N; i++)
    //     verify[i] = ans[ans[i]];
    // puts("-----------");
    // for (int i = 1; i <= N; i++)
    //     printf("%d ", verify[i]);
    // puts("");

    return 0;
}
