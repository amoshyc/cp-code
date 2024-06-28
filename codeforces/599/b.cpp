#include <bits/stdc++.h>
using namespace std;

int N, M;
int pos_of[100000+10];
int a[100000];

int main() {
    memset(pos_of, -1, sizeof(pos_of));

    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        int f_i; scanf("%d", &f_i);
        if (pos_of[f_i] == -1) {
            pos_of[f_i] = i;
        }
        else {
            pos_of[f_i] = -2;
        }
    }

    bool is_possible = true, is_amb = false;
    for (int i = 0; i < M; i++) {
        int b_i; scanf("%d", &b_i);
        if (pos_of[b_i] == -2) {
            is_amb = true;
        }
        else if (pos_of[b_i] == -1) {
            is_possible = false;
        }
        else {
            a[i] = pos_of[b_i];
        }
    }

    if (!is_possible) {
        puts("Impossible");
    }
    else if (is_amb) {
        puts("Ambiguity");
    }
    else {
        puts("Possible");
        for (int i = 0; i < M; i++) {
            if (i != 0) printf(" ");
            printf("%d", a[i] + 1);
        }
        printf("\n");
    }

    return 0;
}
