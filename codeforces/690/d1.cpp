#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 100;
int R, C;
char A[MAX_N][MAX_N + 10];

const int dr[4] = {0, 0, +1, -1};
const int dc[4] = {+1, -1, 0, 0};

void dfs(int r, int c) {
    A[r][c] = '$';
    for (int i = 0; i < 4; i++) {
        int nr = r + dr[i];
        int nc = c + dc[i];
        if (nr < 0 || nr >= R || nc < 0 || nc >= C) continue;
        if (A[nr][nc] == 'B') dfs(nr, nc);
    }
}

int main() {
    scanf("%d %d", &R, &C);
    for (int r = 0; r < R; r++) {
        scanf("%s", A[r]);
    }

    int cnt = 0;
    for (int r = 0; r < R; r++) {
        for (int c = 0; c < C; c++) {
            if (A[r][c] == 'B') {
                cnt++;
                dfs(r, c);
            }
        }
    }

    printf("%d\n", cnt);


    return 0;
}
