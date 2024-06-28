#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int RIGHT = 0;
int DOWN = 1;
int LEFT = 2;
int UP = 3;

int getdir(int x1, int y1, int x2, int y2) {
    if (x1 < x2) return RIGHT;
    if (x1 > x2) return LEFT;
    if (y1 < y2) return UP;
    if (y1 > y2) return DOWN;
    return -1;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    N++;
    vector<int> x(N, 0);
    vector<int> y(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> x[i];
        cin >> y[i];
    }

    int cnt = 0;
    // bool isfirsthalf = true;
    for (int i = 1; i < N - 1; i++) {
        int dir1 = getdir(x[i-1], y[i-1], x[i], y[i]);
        int dir2 = getdir(x[i], y[i], x[i+1], y[i+1]);

        // if (dir1 == RIGHT && dir2 == DOWN)
        //     isfirsthalf = false;

        if (dir1 == RIGHT && dir2 == UP)
            cnt++;
        if (dir1 == LEFT && dir2 == DOWN)
            cnt++;
        if (dir1 == UP && dir2 == LEFT)
            cnt++;
        if (dir1 == DOWN && dir2 == RIGHT)
            cnt++;
    }

    cout << cnt << "\n";

    return 0;
}
