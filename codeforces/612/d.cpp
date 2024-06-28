#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii; // <idx, type>
const int max_n = 1000000;
const int START = 0;
const int END = 1;

int N, K;
pii data[2 * max_n];

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++) {
        scanf("%d", &data[i * 2 + 0].first);
        scanf("%d", &data[i * 2 + 1].first);
        data[i * 2 + 0].second = START;
        data[i * 2 + 1].second = END;
    }
    
    sort(data, data + 2 * N);
    
    vector<pii> ans; // <start, end>
    int start, end;
    int val = 0;
    
    for (int i = 0; i < 2 * N; i++) {
        if (val == K-1 && data[i].second == START) {
            start = data[i].first;
        }
        if (val == K && data[i].second == END) {
            end = data[i].first;
            ans.push_back(pii(start, end));
        }
        
        val += ((data[i].second == START) ? 1 : -1);
    }
    
    int m = ans.size();
    printf("%d\n", m);
    for (int i = 0; i < m; i++)
        printf("%d %d\n", ans[i].first, ans[i].second);
    
    return 0;
}