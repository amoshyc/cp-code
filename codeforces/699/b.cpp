#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii;
#define st first
#define nd second
const int MAX_N = 1000;
int N, M;
char A[MAX_N][MAX_N + 1];
int rr = -1, cc = -1;

bool solve() {
    vector<int> R(N, 0);
    vector<int> C(M, 0);
    int cnt = 0;

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            if (A[r][c] == '*') {
                R[r]++;
                C[c]++;
                cnt++;
            }
        }
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            int cover = R[r] + C[c] - (A[r][c] == '*');
            if (cover == cnt) {
                rr = r;
                cc = c;
                return true;
            }
        }
    }

    return false;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int r = 0; r < N; r++) {
        scanf("%s", A[r]);
    }

    if (solve()) {
        puts("YES");
        printf("%d %d\n", rr + 1, cc + 1);
    }
    else {
        puts("NO");
    }

    return 0;
}
