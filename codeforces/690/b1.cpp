#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 50;
int N;
char A[MAX_N][MAX_N + 10];
char B[MAX_N][MAX_N + 10];

bool solve() {
    int r1 = N, r2 = -1, c1 = N, c2 = -1;
    int cnt = 0;

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            if (A[r][c] == '4') {
                cnt++;
                r1 = min(r1, r);
                c1 = min(c1, c);
                r2 = max(r2, r);
                c2 = max(c2, c);
            }
        }
    }

    // printf("%d: %d, %d, %d, %d\n", cnt, r1, c1, r2, c2);

    if (cnt == 0)
        return false;

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            B[r][c] = '0';
        }
    }

    // 4
    for (int r = r1; r <= r2; r++) {
        for (int c = c1; c <= c2; c++) {
            B[r][c] = '4';
        }
    }
    // 2 left
    if (c1 - 1 >= 0) {
        for (int r = r1; r <= r2; r++) {
            B[r][c1 - 1] = '2';
        }
    }
    // 2 right
    if (c2 + 1 < N) {
        for (int r = r1; r <= r2; r++) {
            B[r][c2 + 1] = '2';
        }
    }
    // 2 top
    if (r1 - 1 >= 0) {
        for (int c = c1; c <= c2; c++) {
            B[r1 - 1][c] = '2';
        }
    }
    // 2 bottom
    if (r2 + 1 < N) {
        for (int c = c1; c <= c2; c++) {
            B[r2 + 1][c] = '2';
        }
    }
    // 1 top-left
    if (r1 - 1 >= 0 && c1 - 1 >= 0) {
        B[r1 - 1][c1 - 1] = '1';
    }
    // 1 top-right
    if (r2 + 1 < N && c1 - 1 < N) {
        B[r2 + 1][c1 - 1] = '1';
    }
    // 1 bottom-left
    if (r1 - 1 >= 0 && c2 + 1 < N) {
        B[r1 - 1][c2 + 1] = '1';
    }
    // 1 bottom-right
    if (r2 + 1 < N && c2 + 1 >= 0) {
        B[r2 + 1][c2 + 1] = '1';
    }

    // for (int r = 0; r < N; r++) {
    //     printf("%s\n", B[r]);
    // }

    // check
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            if (A[r][c] != B[r][c]) {
                return false;
            }
        }
    }

    return true;
}

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%s", A[i]);
    }

    puts((solve() ? "Yes" : "No"));

    return 0;
}
