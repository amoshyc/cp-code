#include <algorithm>
#include <cstdio>
#include <iomanip>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

int dr[] = {-1, +1, 0, 0};
int dc[] = {0, 0, -1, +1};

char A[2222][2222];
int dis[2222][2222];
bool vis[2222][2222];

int main() {
    int H, W;
    scanf("%d %d", &H, &W);

    for (int r = 0; r < H; r++) {
        for (int c = 0; c < W; c++) {
            dis[r][c] = -1;
            vis[r][c] = false;
        }
    }

    int sr, sc;
    int gr, gc;
    vector<tuple<int, int>> jumps[26];
    for (int r = 0; r < H; r++) {
        scanf("%s", A[r]);
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

    dis[sr][sc] = 0;
    vis[sr][sc] = true;
    que.push({sr, sc, 0});

    while (!que.empty()) {
        auto [r, c, d] = que.front();
        que.pop();

        if (r == gr and c == gc)
            break;

        for (int i = 0; i < 4; i++) {
            int nr = r + dr[i];
            int nc = c + dc[i];
            if (nr < 0 or nr >= H)
                continue;
            if (nc < 0 or nc >= W)
                continue;
            if (A[nr][nc] == '#')
                continue;
            if (vis[nr][nc])
                continue;
            dis[nr][nc] = d + 1;
            vis[nr][nc] = true;
            que.push({nr, nc, d + 1});
        }

        if ('a' <= A[r][c] and A[r][c] <= 'z') {
            for (auto [nr, nc] : jumps[A[r][c] - 'a']) {
                if (!vis[nr][nc]) {
                    dis[nr][nc] = d + 1;
                    vis[nr][nc] = true;
                    que.push({nr, nc, d + 1});
                }
            }
        }
    }

    // cout << dis[gr][gc] << endl;
    printf("%d\n", dis[gr][gc]);

    return 0;
}