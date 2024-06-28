#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    scanf("%d", &N);
    vector<int> a(N, 0);
    for (int i = 0 ; i < N; i++)
        scanf("%d", &a[i]);
    int mx = *max_element(a.begin(), a.end());
    int cnt = 0;
    for (int x : a) {
        cnt += (mx - x);
    }
    printf("%d\n", cnt);
    return 0;
}
