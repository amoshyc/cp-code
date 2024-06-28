#include <bits/stdc++.h>
using namespace std;

int gcd(int a, int b) {
    while (b) {
        // a, b = b, a % b
        int temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

int main() {
    int N;
    scanf("%d", &N);
    
    int ans; scanf("%d", &ans);
    for (int i = 1; i < N; i++) {
        int inp; scanf("%d", &inp);
        ans = gcd(ans, inp);
    }
    
    printf("%d\n", ans * N);
    
    return 0;
}