#include <vector>
#include <queue>
#include <iostream>
#include <tuple>
#include <cmath>
using namespace std;


template <typename T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << ", ";
    }
    out << "]";
    return out;
}


int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, M;
    cin >> N >> M;

    const int INF = 0x3f3f3f3f;
    auto d = vector(N, vector<int>(N, INF));
    auto que = queue<tuple<int, int>>();

    d[0][0] = 0;
    que.push({0, 0});

    while (que.size() > 0) {
        auto [r, c] = que.front();
        que.pop();

        auto edges = vector<tuple<int, int>>();
        for (int nr = 0; nr < N; nr++) {
            if (M < (r - nr) * (r - nr)) {
                continue;
            }
            int nc1 = c - sqrt(M - (r - nr) * (r - nr));
            if (nc1 >= 0 && nc1 < N && (r - nr) * (r - nr) + (c - nc1) * (c - nc1) == M) {
                edges.push_back({nr, nc1});
            }
            int nc2 = c + sqrt(M - (r - nr) * (r - nr));
            if (nc2 >= 0 && nc2 < N && (r - nr) * (r - nr) + (c - nc2) * (c - nc2) == M) {
                edges.push_back({nr, nc2});
            }
        }
        for (int nc = 0; nc < N; nc++) {
            if ((r - r) * (r - r) + (c - nc) * (c - nc) == M) {
                edges.push_back({r, nc});
            }
        }

        for (auto [nr, nc] : edges) {
            if (d[nr][nc] == INF) {
                d[nr][nc] = d[r][c] + 1;
                que.push({nr, nc});
            }
        }
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            cout << ((d[r][c] == INF) ? -1 : d[r][c]) << " ";
        }
        cout << endl;
    }

    return 0;
}