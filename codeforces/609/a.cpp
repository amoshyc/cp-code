#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        scanf("%d", &data[i]);
    
    sort(data.begin(), data.end());
    reverse(data.begin(), data.end());
    int cnt = 0;
    for (int c : data) {
        M -= c;
        cnt++;
        
        if (M <= 0) break;
    }
    
    printf("%d\n", cnt);
    
    return 0;
}