#include <iostream>
#include <cstdio>
using namespace std;

int main() {
    int N, X;
    scanf("%d %d", &N, &X);
    int cnt = 0;
    for (int i = 1; i <= N; i++) {
        if (X % i == 0 && X / i <= N)
            cnt++;
    }
    printf("%d\n", cnt);

    return 0;
}
