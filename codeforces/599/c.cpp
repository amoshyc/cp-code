#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int N;
ll data[100000];
ll prefmax[100000];
ll suffmin[100000 + 1];

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++)
        scanf("%I64d", &data[i]);
    
    prefmax[0] = data[0];
    for (int i = 1; i < N; i++)
        prefmax[i] = max(prefmax[i-1], data[i]);
    
    suffmin[N-1] = data[N-1];
    suffmin[N] = 1e10;
    for (int i = N - 2; i >= 0; i--)
        suffmin[i] = min(suffmin[i+1], data[i]);
    
    int cnt = 0;
    for (int i = 0; i < N; i++)
        if (prefmax[i] <= suffmin[i+1])
            cnt++;
            
    printf("%d\n", cnt);
    
    return 0;
}