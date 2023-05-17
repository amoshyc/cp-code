#include <iostream>
#include <vector>
#include <queue>
#include <tuple>
#include <functional>
#include <algorithm>
using namespace std;

typedef tuple<int, int, int> t3i;
const int INF = 0x3f3f3f3f;

int solve() {
    int H, W, K;
    cin >> H >> W >> K;

    int sr, sc, tr, tc;
    cin >> sr >> sc >> tr >> tc;
    sr--; sc--; tr--; tc--;

    vector<string> A(H);
    for (int r = 0; r < H; r++) {
        cin >> A[r];
    }

    auto deltas = vector<t3i>();
    deltas.push_back({+1, 0, 1});
    deltas.push_back({-1, 0, 1});
    deltas.push_back({0, +1, 1});
    deltas.push_back({0, -1, 1});

    auto dis = vector<vector<int>>(H, vector<int>(W, INF));
    auto que = priority_queue<t3i, vector<t3i>, greater<t3i>>();

    dis[sr][sc] = 0;
    que.push({0, sr, sc});

    while (!que.empty()) {
        auto [d, r, c] = que.top(); que.pop();
        if (d > dis[r][c]) continue;
        for (auto&& [dr, dc, w]: deltas) {
            for (int k = 1; k <= K; k++) {
                int nr = r + k * dr;
                int nc = c + k * dc;
                if (nr < 0 || nr >= H) break;
                if (nc < 0 || nc >= W) break;
                if (A[nr][nc] == '@') break;
                if (dis[nr][nc] < d + w) break;
                if (dis[nr][nc] > d + w) {
                    dis[nr][nc] = d + w;
                    que.push({dis[nr][nc], nr, nc});
                }
            }
        }
    }

    return ((dis[tr][tc] == INF) ? -1: dis[tr][tc]);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}