#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    
    vector<int> cnt(M, 0);
    for (int i = 0; i < N; i++) {
        int g; scanf("%d", &g);
        cnt[g - 1]++;
    }
    
    int total = 0;
    for (int i = 0; i < M - 1; i++) {
        for (int j = i + 1; j < M; j++) {
            total += cnt[i] * cnt[j];
        }
    }
    
    printf("%d\n", total);
    
    return 0;
}