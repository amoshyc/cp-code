#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

// 1-based index
const int MAX_N = 100;
int N;
int nxt[MAX_N + 1];
int dep[MAX_N + 1];
int in[MAX_N + 1];
vector<int> cycles;

void dfs(int u, int d) {
    dep[u] = d;

    int v = nxt[u];
    if (dep[v] == -1)  {
        dfs(v, d + 1);
    }
    else {
        int len = dep[u] - dep[v] + 1;
        cycles.push_back(((len % 2 == 0) ? len / 2 : len));
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    scanf("%d", &N);
    for (int u = 1; u <= N; u++) {
        scanf("%d", &nxt[u]);
        in[nxt[u]]++;
    }

    if (count(in + 1, in + N + 1, 1) != N) {
        puts("-1");
        return 0;
    }

    fill(dep + 1, dep + N + 1, -1);
    for (int u = 1; u <= N; u++) {
        if (dep[u] == -1) {
            dfs(u, 0);
        }
    }

    int ans = 1;
    for (int len : cycles) {
        ans = ans / __gcd(ans, len) * len;
    }
    cout << ans << endl;

    return 0;
}
