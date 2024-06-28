#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 1e5;
int N, cnt;
char A[MAX_N + 1];

int id(char c) {
    int id = -1;
    if (isupper(c))
        id = c - 'A';
    else
        id = c - 'a' + 26;
    return id;
}

bool C(int len) {
    vector<int> vis(52, 0);
    int vis_cnt = 0;
    for (int i = 0; i < len; i++) {
        vis[id(A[i])]++;
        if (vis[id(A[i])] == 1)
            vis_cnt++;
    }

    if (vis_cnt >= cnt)
        return true;

    for (int s = 1; s + len - 1 < N; s++) {
        int t = s + len - 1;

        vis[id(A[s - 1])]--;
        if (vis[id(A[s - 1])] == 0) vis_cnt--;

        vis[id(A[t])]++;
        if (vis[id(A[t])] == 1) vis_cnt++;

        if (vis_cnt >= cnt)
            return true;
    }

    return false;
}

int solve() {
    vector<int> vis(52, 0);
    for (int i = 0; i < N; i++)
        vis[id(A[i])] = 1;
    cnt = accumulate(vis.begin(), vis.end(), 0);

    // 0 0 0 1 1 1
    int lb = 1, ub = N;
    if (C(lb)) return lb;
    // if (!C(ub)) impossible;
    while (ub - lb > 1) {
        int mid = (lb + ub) / 2;
        if (C(mid)) ub = mid;
        else lb = mid;
    }
    return ub;
}

int main() {
    scanf("%d", &N);
    scanf("%s", A);

    printf("%d\n", solve());

    return 0;
}
