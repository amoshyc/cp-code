#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    scanf("%d", &N);

    int cnt = 0;
    while (N != 0) {
        if (N & 1)
            cnt++;
        N = N >> 1;
    }

    printf("%d\n", cnt);

    return 0;
}
