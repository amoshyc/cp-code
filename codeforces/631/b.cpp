#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 5000;
const int MAX_M = 5000;
const int MAX_K = 100000;

int N, M, K;
int r_op[MAX_N];
int c_op[MAX_M];
int op[MAX_K + 1];

int main() {
    scanf("%d %d %d", &N, &M, &K);

    op[0] = 0;

    for (int i = 1; i <= K; i++) {
        int cmd, pos, c;
        cin >> cmd >> pos >> c;
        pos--;

        if (cmd == 1) {
            r_op[pos] = i;
        }
        else {
            c_op[pos] = i;
        }

        op[i] = c;
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            int last_op = max(r_op[r], c_op[c]);
            printf("%d ", op[last_op]);
        }
        puts("");
    }

    return 0;
}
