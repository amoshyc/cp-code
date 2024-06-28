#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int A[49][49];

int main() {
    int N;
    scanf("%d", &N);

    vector<int> odd;
    vector<int> even;
    for (int i = 1; i <= N * N; i++) {
        if (i & 1)
            odd.push_back(i);
        else
            even.push_back(i);
    }

    for (int r = 0; r < N; r++) {
        A[r][N / 2] = odd.back();
        odd.pop_back();
    }
    for (int c = 0; c < N; c++) {
        if (c == N / 2) continue;
        A[N / 2][c] = odd.back();
        odd.pop_back();
    }

    // for (int r = 0; r < N; r++) {
    //     for (int c = 0; c < N; c++) {
    //         printf("%2d ", A[r][c]);
    //     }
    //     puts("");
    // }
    //
    // puts("----------------");

    bool empty = odd.empty();
    for (int r = 0; r < N / 2 && !empty; r++) {
        for (int c = 0; c < N / 2 && !empty; c++) {
            A[r][c] = odd.back(); odd.pop_back();
            A[r][N - c - 1] = odd.back(); odd.pop_back();
            A[N - r - 1][c] = odd.back(); odd.pop_back();
            A[N - r - 1][N - c - 1] = odd.back(); odd.pop_back();

            empty = odd.empty();
        }
    }

    // for (int r = 0; r < N; r++) {
    //     for (int c = 0; c < N; c++) {
    //         printf("%2d ", A[r][c] % 2);
    //     }
    //     puts("");
    // }
    //
    // puts("-----------------");

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            if (A[r][c] == 0) {
                A[r][c] = even.back();
                even.pop_back();
            }
        }
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            printf("%d ", A[r][c]);
        }
        puts("");
    }

    return 0;
}
