#include <cmath>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>
#include <iomanip>
using namespace std;

using P = pair<double, int>;
using C = tuple<double, double, double>;


template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << " ";
    }
    out << "]";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    double xs, ys, xt, yt;
    cin >> xs >> ys >> xt >> yt;

    int N;
    cin >> N;
    vector<C> circles;
    for (int i = 0; i < N; i++) {
        double x, y, r;
        cin >> x >> y >> r;
        circles.push_back({x, y, r});
    }

    circles.push_back({xs, ys, 0});
    circles.push_back({xt, yt, 0});
    int S = N;
    int T = N + 1;

    auto w = vector<vector<double>>(N + 2, vector<double>(N + 2, -1));
    for (int i = 0; i < N + 2; i++) {
        for (int j = 0; j < N + 2; j++) {
            auto [xi, yi, ri] = circles[i];
            auto [xj, yj, rj] = circles[j];
            w[i][j] = sqrt((xi - xj) * (xi - xj) + (yi - yj) * (yi - yj));
            w[i][j] = max(w[i][j] - (ri + rj), 0.0);
        }
    }

    auto que = priority_queue<P, vector<P>, greater<P>>();
    auto dis = vector<double>(N + 2, 1e15);

    dis[S] = 0.0;
    que.push({dis[S], S});

    while (!que.empty()) {
        auto [d, u] = que.top();
        que.pop();

        if (dis[u] < d) {
            continue;
        }

        for (int v = 0; v < N + 2; v++) {
            if (v != u && dis[v] > dis[u] + w[u][v]) {
                dis[v] = dis[u] + w[u][v];
                que.push({dis[v], v});
            }
        }
    }

    cout << fixed << setprecision(15) << dis[T] << endl;

    return 0;
}