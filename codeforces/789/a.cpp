#include <bits/stdc++.h>
using namespace std;

inline int ceildiv(int a, int b) {
    return (a + b - 1) / b;
}

int main() {
    int n, k;
    scanf("%d %d", &n, &k);

    int cnt = 0;
    for (int i = 0; i < n; i++) {
        int w; scanf("%d", &w);
        cnt += ceildiv(w, k);
    }

    printf("%d\n", ceildiv(cnt, 2));

    return 0;
}
