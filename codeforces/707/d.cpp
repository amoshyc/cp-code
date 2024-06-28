#include <bits/stdc++.h>
using namespace std;

struct Query {
    int id, cmd, i, j, ans;
};

const int MAX_N = 1000;
const int MAX_M = 1000;
const int MAX_Q = 100000;

int N, M, Q;
bool A[MAX_N][MAX_M];
vector<int> g[MAX_Q + 1];
vector<Query> query;

int cnt = 0;
int cntR[MAX_N];
bool inv[MAX_N];

inline bool get(int i, int j) {
    return A[i][j] ^ inv[i];
}

bool apply(const Query& q) {
    bool state = false;
    if (q.cmd == 1) {
        state = get(q.i, q.j);
        if (state == false) {
            cnt++;
            cntR[q.i]++;
            A[q.i][q.j] ^= 1;
        }
    }
    if (q.cmd == 2) {
        state = get(q.i, q.j);
        if (state == true) {
            cnt--;
            cntR[q.i]--;
            A[q.i][q.j] ^= 1;
        }
    }
    if (q.cmd == 3) {
        inv[q.i] ^= 1;
        cnt -= cntR[q.i];
        cntR[q.i] = M - cntR[q.i];
        cnt += cntR[q.i];
    }
    return state;
}

void undo(const Query& q, bool state) {
    if (q.cmd == 1) {
        if (state == false) {
            cnt--;
            cntR[q.i]--;
            A[q.i][q.j] ^= 1;
        }
    }
    if (q.cmd == 2) {
        if (state == true) {
            cnt++;
            cntR[q.i]++;
            A[q.i][q.j] ^= 1;
        }
    }
    if (q.cmd == 3) {
        inv[q.i] ^= 1;
        cnt -= cntR[q.i];
        cntR[q.i] = M - cntR[q.i];
        cnt += cntR[q.i];
    }
}

void dfs(int u) {
    bool originState = apply(query[u]);

    query[u].ans = cnt;
    for (int v : g[u]) {
        dfs(v);
    }

    undo(query[u], originState);
}

int main () {
    scanf("%d %d %d", &N, &M, &Q);

    query.push_back((Query) {0, -1, -1, -1, -1});
    for (int id = 1; id <= Q; id++) {
        int cmd; scanf("%d", &cmd);
        if (cmd <= 2) {
            int i, j; scanf("%d %d", &i, &j); i--; j--;
            query.push_back((Query) {id, cmd, i, j, -1});
            g[id - 1].push_back(id);
        }
        else if (cmd == 3) {
            int i; scanf("%d", &i); i--;
            query.push_back((Query) {id, cmd, i, -1, -1});
            g[id - 1].push_back(id);
        }
        else {
            int k; scanf("%d", &k);
            query.push_back((Query) {id, cmd, -1, -1, -1});
            g[k].push_back(id);
        }
    }

    dfs(0);
    for (int i = 1; i <= Q; i++)
        printf("%d\n", query[i].ans);

    return 0;
}
