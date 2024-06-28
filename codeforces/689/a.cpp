#include <bits/stdc++.h>
using namespace std;

int N;
int dr[10];
int dc[10];

int r[10] = {3, 0, 0, 0, 1, 1, 1, 2, 2, 2};
int c[10] = {1, 0, 1, 2, 0, 1, 2, 0, 1, 2};

bool P(int s) {
    int cur_r = r[s], cur_c = c[s];
    for (int i = 1; i < N; i++) {
        cur_r = cur_r + dr[i];
        cur_c = cur_c + dc[i];

        if (cur_r < 0 || cur_r >= 4) return false;
        if (cur_c < 0 || cur_c >= 3) return false;
        if (cur_r == 3 && cur_c != 1) return false;
    }
    return true;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string inp;
    cin >> N >> inp;
    for (int i = 1; i < N; i++) {
        dr[i] = r[inp[i] - '0'] - r[inp[i - 1] - '0'];
        dc[i] = c[inp[i] - '0'] - c[inp[i - 1] - '0'];

        // cout << r[inp[i] - '0'] << ", " << r[inp[i - 1] - '0'] << endl;
        // cout << c[inp[i] - '0'] << ", " << c[inp[i - 1] - '0'] << endl;
    }

    int cnt = 0;
    for (int i = 0; i <= 9; i++) {
        if (P(i)) {
            cnt++;
        }
    }

    if (cnt == 1) cout << "YES\n";
    else {
        cout << "NO\n";
    }

    return 0;
}
