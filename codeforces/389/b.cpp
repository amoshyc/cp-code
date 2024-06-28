#include <bits/stdc++.h>
using namespace std;

const int max_n = 100;
int N;
char data[max_n][max_n + 1];

const int dr[4] = {+1, +1, +1, +2};
const int dc[4] = {-1, 0, +1, 0};

bool possible() {
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            if (data[r][c] == '#') {
                for (int i = 0; i < 4; i++) {
                    int nr = r + dr[i];
                    int nc = c + dc[i];
                    if (nr < 0 || nr >= N || nc < 0 || nc >= N)
                        return false;
                    if (data[nr][nc] != '#')
                        return false;
                    data[nr][nc] = '.';
                }
            }
        }
    }
    
    return true;
}

int main() {
    scanf("%d", &N);
    for (int r = 0; r < N; r++) {
        scanf("%s", data[r]);
    }
    
    if (possible())
        puts("YES");
    else
        puts("NO");
    
    return 0;
}