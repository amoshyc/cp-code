#include <algorithm>
#include <iomanip>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

int dr[] = {-1, +1, 0, 0};
int dc[] = {0, 0, -1, +1};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int H, W;
    cin >> H >> W;

    int sr, sc;
    int gr, gc;
    vector<tuple<int, int>> jumps[26];
    vector<string> A(H);
    for (int r = 0; r < H; r++) {
        cin >> A[r];
        for (int c = 0; c < W; c++) {
            if (A[r][c] == 'S') {
                sr = r;
                sc = c;
            } else if (A[r][c] == 'G') {
                gr = r;
                gc = c;
            } else if ('a' <= A[r][c] and A[r][c] <= 'z') {
                jumps[A[r][c] - 'a'].push_back({r, c});
            }
        }
    }

    queue<tuple<int, int, int>> que;
    vector<vector<int>> dis(H, vector<int>(W, -1));

    dis[sr][sc] = 0;
    que.push({sr, sc, 0});

    while (!que.empty()) {
        auto [r, c, d] = que.front();
        que.pop();

        for (int i = 0; i < 4; i++) {
            int nr = r + dr[i];
            int nc = c + dc[i];
            if (nr < 0 or nr >= H)
                continue;
            if (nc < 0 or nc >= W)
                continue;
            if (A[nr][nc] == '#')
                continue;
            if (dis[nr][nc] != -1)
                continue;
            dis[nr][nc] = d + 1;
            que.push({nr, nc, d + 1});
        }

        if ('a' <= A[r][c] and A[r][c] <= 'z') {
            for (auto [nr, nc] : jumps[A[r][c] - 'a']) {
                if (dis[nr][nc] == -1) {
                    dis[nr][nc] = d + 1;
                    que.push({nr, nc, d + 1});
                }
            }
            jumps[A[r][c] - 'a'].clear();
        }
    }

    cout << dis[gr][gc] << endl;

    return 0;
}