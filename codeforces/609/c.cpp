#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    scanf("%d", &N);
    
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        scanf("%d", &data[i]);
        
    sort(data.begin(), data.end());
    
    int sum = accumulate(data.begin(), data.end(), 0);
    vector<int> flag(N, sum / N);
    for (int i = 0; i < (sum % N); i++)
        flag[N - 1 - i] = sum / N + 1;
    
    long long cnt = 0;
    for (int i = 0; i < N; i++) {
        cnt += abs(data[i] - flag[i]);
    }
    cnt /= 2;
    
    printf("%I64d\n", cnt);
    
    return 0;
}