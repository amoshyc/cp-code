#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>
#include <tuple>
#include <functional>
using namespace std;

typedef pair<int, int> pii;
const int inf = 0x3f3f3f3f;

int solve() {
    int H, W; 
    int Cr, Cc;
    int Dr, Dc;
    cin >> H >> W;
    cin >> Cr >> Cc; Cr--; Cc--;
    cin >> Dr >> Dc; Dr--; Dc--;
    vector<string> S(H);
    for (int r = 0; r < H; r++) {
        cin >> S[r];
    }

    if (S[Cr][Cc] == '#' || S[Dr][Dc] == '#') {
        return -1;
    }

    auto deltas = vector<tuple<int, int, int>>();
    deltas.push_back(tuple(0, +1, 0));
    deltas.push_back(tuple(0, -1, 0));
    deltas.push_back(tuple(+1, 0, 0));
    deltas.push_back(tuple(-1, 0, 0));
    for (int dr = -2; dr <= 2; dr++) {
        for (int dc = -2; dc <= 2; dc++) {
            if (dr == 0 and dc == 0) continue;
            deltas.push_back(tuple(dr, dc, 1));
        }
    }

    auto dis = vector<int>(H * W, inf);
    auto que = priority_queue<pii, vector<pii>, greater<pii>>();

    dis[Cr * W + Cc] = 0;
    que.push(pii(0, Cr * W + Cc));

    while (!que.empty()) {
        auto [d, u] = que.top();
        que.pop();

        if (dis[u] < d) continue;

        for (auto&& [dr, dc, w] : deltas) {
            auto nr = u / W + dr;
            auto nc = u % W + dc;
            if (nr < 0 || nr >= H) continue;
            if (nc < 0 || nc >= W) continue;
            if (S[nr][nc] == '#') continue;
            if (dis[nr * W + nc] > dis[u] + w) {
                dis[nr * W + nc] = dis[u] + w;
                que.push(pii(dis[nr * W + nc], nr * W + nc));
            }
        }
    }

    if (dis[Dr * W + Dc] == inf) {
        return -1;
    }
    return dis[Dr * W + Dc];
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}