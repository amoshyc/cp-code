#include <bits/stdc++.h>
using namespace std;

int N, M, K;
int data[1000][1000];
int look_at[1000][1000];
int ans[1000][1000];
const int dr[4] = {0, 1, 0, -1};
const int dc[4] = {1, 0, -1, 0};

char input[1000 + 1];

int dfs(int row, int col, const int flag) {
    look_at[row][col] = flag;

    int cnt = 0;
    for (int i = 0; i < 4; i++) {
        int nr = row + dr[i];
        int nc = col + dc[i];
        if (0 <= nr && nr < N && 0 <= nc && nc < M) {
            if (data[nr][nc] == 1) {
                cnt++;
            }
            else if (look_at[nr][nc] == -1){
                cnt += dfs(nr, nc, flag);
            }
        }
    }
    return cnt;
}

int main() {
    scanf("%d %d %d", &N, &M, &K);
    for (int row = 0; row < N; row++) {
        scanf("%s", input);
        for (int col = 0; col < M; col++) {
            data[row][col] = ((input[col] == '*') ? 1 : 0);
        }
    }

    // build
    memset(look_at, -1, sizeof(look_at));
    for (int row = 0; row < N; row++) {
        for (int col = 0; col < M; col++) {
            if (data[row][col] == 0 && look_at[row][col] == -1) {
                int cnt = dfs(row, col, row * M + col);
                ans[row][col] = cnt;

                // printf("Set %d %d = %d\n", row, col, cnt);
            }
        }
    }

    while (K--) {
        int row, col; scanf("%d %d", &row, &col);
        row--; col--;
        int flag = look_at[row][col];
        // printf("%d: %d, %d\n", flag, flag / M, flag % M);
        printf("%d\n", ans[flag / M][flag % M]);
    }

    return 0;
}
