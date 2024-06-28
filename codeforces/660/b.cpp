#include <bits/stdc++.h>
using namespace std;

int data[100][4];

int main() {
    int N, M;
    cin >> N >> M;

    int lw = 1, rw = 2, ln = 2 * N + 1, rn = 2 * N + 2;
    for (int r = 0; r < N; r++) {
        data[r][0] = lw; lw += 2;
        data[r][3] = rw; rw += 2;
        data[r][1] = ln; ln += 2;
        data[r][2] = rn; rn += 2;
    }

    // for (int r = 0; r < N; r++) {
    //     for (int c = 0; c < 4; c++) {
    //         cout << data[r][c] << " ";
    //     }
    //     cout << endl;
    // }

    for (int r = 0; r < N; r++) {
        if (data[r][1] <= M) cout << data[r][1] << " ";
        if (data[r][0] <= M) cout << data[r][0] << " ";
        if (data[r][2] <= M) cout << data[r][2] << " ";
        if (data[r][3] <= M) cout << data[r][3] << " ";
    }
    cout << endl;

    return 0;
}
