#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N, Bx;
    scanf("%d %d", &N, &Bx);
    ll X = 0;
    for (int i = 0; i < N; i++) {
        int val; scanf("%d", &val);
        X = X * Bx + val;
    }

    int M, By;
    scanf("%d %d", &M, &By);
    ll Y = 0;
    for (int i = 0; i < M; i++) {
        int val; scanf("%d", &val);
        Y = Y * By + val;
    }

    // printf("%d %d\n", X, Y);

    if (X > Y)
        printf(">\n");
    else if (X < Y)
        printf("<\n");
    else
        printf("=\n");

    return 0;
}
