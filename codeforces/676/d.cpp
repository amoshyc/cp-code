#include <bits/stdc++.h>
using namespace std;

typedef tuple<int, int, int, int> t4i; // <r, c, rot, level>
const int dr[4] = {-1, +1, 0, 0};
const int dc[4] = {0, 0, -1, +1};

const int T = 0b1000;
const int D = 0b0100;
const int L = 0b0010;
const int R = 0b0001;

const int MAX_N = 1000;
const int MAX_M = 1000;
int N, M;
int maze[MAX_N][MAX_M];
bool vis[MAX_N][MAX_M][4];
int SR, SC; // start position
int TR, TC; // target position

inline int get_val(char c) {
    if (c == '+') return (T | D | L | R);
    if (c == '-') return (L | R);
    if (c == '|') return (T | D);
    if (c == '^') return (T);
    if (c == '<') return (L);
    if (c == '>') return (R);
    if (c == 'v') return (D);
    if (c == 'L') return (T | D | R);
    if (c == 'R') return (T | D | L);
    if (c == 'U') return (D | L | R);
    if (c == 'D') return (T | L | R);
    return 0;
}

inline int rotate(int val) {
    int res = 0;
    if (val & T) res |= R;
    if (val & R) res |= D;
    if (val & D) res |= L;
    if (val & L) res |= T;
    return res;
}

bool connect(int r, int c, int rot, int dir, int nr, int nc) {
    rot %= 4;
    int val1 = maze[r][c];
    for (int _ = 0; _ < rot; _++)
        val1 = rotate(val1);
    int val2 = maze[nr][nc];
    for (int _ = 0; _ < rot; _++)
        val2 = rotate(val2);

    if (dir == 0) { // T
        return (val1 & T) && (val2 & D);
    }
    if (dir == 1) { // D
        return (val1 & D) && (val2 & T);
    }
    if (dir == 2) { // L
        return (val1 & L) && (val2 & R);
    }
    if (dir == 3) { // R
        return (val1 & R) && (val2 & L);
    }

    return false;
}

int bfs() {
    memset(vis, false, sizeof(vis));
    queue<t4i> q;

    q.push(t4i(SR, SC, 0, 0));
    vis[SR][SC][0] = true;

    while (!q.empty()) {
        t4i v = q.front(); q.pop();

        int r = get<0>(v);
        int c = get<1>(v);
        int rot = get<2>(v);
        int level = get<3>(v);

        // printf("%d, %d, %d, %d\n", r, c, rot, level);

        if (r == TR && c == TC)
            return level;

        // neighbors
        for (int dir = 0; dir < 4; dir++) {
            int nr = r + dr[dir];
            int nc = c + dc[dir];
            if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
            if (!vis[nr][nc][rot % 4] && connect(r, c, rot, dir, nr, nc)) {
                vis[nr][nc][rot % 4] = true;
                q.push(t4i(nr, nc, rot, level + 1));
            }
        }

        // rotate
        if (!vis[r][c][(rot + 1) % 4]) {
            vis[r][c][(rot + 1) % 4] = true;
            q.push(t4i(r, c, rot + 1, level + 1));
        }
    }

    return -1;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int r = 0; r < N; r++) {
        char inp[MAX_M + 1];
        scanf("%s", inp);
        for (int c = 0; c < M; c++) {
            maze[r][c] = get_val(inp[c]);
        }
    }
    scanf("%d %d", &SR, &SC); SC--; SR--;
    scanf("%d %d", &TR, &TC); TC--; TR--;

    printf("%d\n", bfs());

    return 0;
}
