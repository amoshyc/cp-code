#include <bits/stdc++.h>
using namespace std;

const int max_n = 1000;
int N, M;
int s[max_n];
int g[max_n];

int solve() {
    for (int i = 0; i < N; i++) {
        if (s[i] < g[i]) {
            return -1;
        }
    }

    set<int> moves;
    for (int i = 0; i < N; i++) {
        int cur_line_move = s[i] - g[i];
        moves.insert(cur_line_move);
    }

    return moves.size();
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        char data[max_n + 1];
        scanf("%s", data);
        for (int j = 0; j < M; j++) {
            if (data[j] == 'S')
                s[i] = j;
            if (data[j] == 'G')
                g[i] = j;
        }
    }

    printf("%d\n", solve());

    return 0;
}
