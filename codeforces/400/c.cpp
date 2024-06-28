#include <bits/stdc++.h>
using namespace std;

int N, M, A, B, C, P;

int main() {
    scanf("%d %d %d %d %d %d", &N, &M, &A, &B, &C, &P);

    A %= 4;
    B %= 2;
    C %= 4;

    // printf("%d %d %d\n", A, B, C);

    while (P--) {
        int Np = N, Mp = M; // N', M'

        int r, c;
        scanf("%d %d", &r, &c);
        c--; r--;

        // printf("0: %d, %d\n", r, c);

        // cw A times
        for (int i = 0; i < A; i++) {
            int nr = c;
            int nc = Np - r - 1;

            r = nr;
            c = nc;
            swap(Np, Mp);
        }

        // printf("1: %d, %d\n", r, c);

        // flip B times
        for (int i = 0; i < B; i++) {
            int nr = r;
            int nc = Mp - c - 1;

            r = nr;
            c = nc;
        }

        // printf("2: %d, %d\n", r, c);

        // ccw C times
        for (int i = 0; i < C; i++) {
            int nr = Mp - c - 1;
            int nc = r;

            r = nr;
            c = nc;
            swap(Np, Mp);
        }

        // printf("3: %d, %d\n", r, c);

        printf("%d %d\n", r + 1, c + 1);
    }

    return 0;
}
