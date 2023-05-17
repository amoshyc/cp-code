#include <iostream>
#include <queue>
#include <tuple>
#include <utility>
#include <vector>
using namespace std;

const int INF = 0x3f3f3f3f;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    int ax, ay;
    cin >> ax >> ay;
    ax--;
    ay--;

    int bx, by;
    cin >> bx >> by;
    bx--;
    by--;

    vector<string> S(N, "");
    for (int i = 0; i < N; i++) {
        cin >> S[i];
    }

    vector<pair<int, int>> dirs = {{+1, +1}, {+1, -1}, {-1, +1}, {-1, -1}};
    queue<pair<int, int>> que;
    vector<vector<int>> dis(N, vector<int>(N, INF));
    que.push({ax, ay});
    dis[ax][ay] = 0;

    while (que.size() > 0) {
        auto [x, y] = que.front();
        que.pop();

        if (x == bx && y == by) {
            cout << dis[x][y] << "\n";
            return 0;
        }

        for (auto [dx, dy] : dirs) {
            for (int i = 1; i < N; i++) {
                int nx = x + i * dx;
                int ny = y + i * dy;
                if (nx < 0 || nx >= N)
                    break;
                if (ny < 0 || ny >= N)
                    break;
                if (S[nx][ny] == '#')
                    break;
                if (dis[nx][ny] < dis[x][y] + 1)
                    break;
                if (dis[nx][ny] == INF) {
                    que.push({nx, ny});
                    dis[nx][ny] = dis[x][y] + 1;
                }
            }
        }
    }

    cout << "-1\n";

    return 0;
}