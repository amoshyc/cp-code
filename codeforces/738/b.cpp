#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 1000;
const int MAX_M = 1000;
int N, M;
int A[MAX_N][MAX_M];

int solve() {
    int cnt = 0;
    // UP
    for (int col = 0; col < M; col++) {
        int row = 0; // first row that has actor
        while (row < N && A[row][col] == 0)
            row++;

        int cnt_empty = 0;
        for (int r = row; r < N; r++) {
            if (A[r][col] == 0) {
                cnt_empty++;
            }
        }

        cnt += cnt_empty;
    }
    // DOWN
    for (int col = 0; col < M; col++) {
        int row = N - 1; // last row that has actor
        while (row >= 0 && A[row][col] == 0)
            row--;

        int cnt_empty = 0;
        for (int r = row; r >= 0; r--) {
            if (A[r][col] == 0) {
                cnt_empty++;
            }
        }

        cnt += cnt_empty;
    }
    // RIGHT
    for (int row = 0; row < N; row++) {
        int col = 0; // fist col that has actor
        while (col < M && A[row][col] == 0)
            col++;

        int cnt_empty = 0;
        for (int c = col; c < M; c++) {
            if (A[row][c] == 0) {
                cnt_empty++;
            }
        }

        cnt += cnt_empty;
    }
    // LEFT
    for (int row = 0; row < N; row++) {
        int col = M - 1; // last col that has actor
        while (col >= 0 && A[row][col] == 0)
            col--;

        int cnt_empty = 0;
        for (int c = col; c >= 0; c--) {
            if (A[row][c] == 0) {
                cnt_empty++;
            }
        }

        cnt += cnt_empty;
    }
    return cnt;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    scanf("%d %d", &N, &M);
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            scanf("%d", &A[r][c]);
        }
    }

    printf("%d\n", solve());

    return 0;
}
