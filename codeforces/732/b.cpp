#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    ll n, k;
    cin >> n >> k;
    
    vector<int> A(n, 0);
    for (int i = 0; i < n; i++)
        cin >> A[i];
        
    int cnt = 0;
    for (int i = 1; i < n; i++) {
        if (A[i - 1] + A[i] < k) {
            int d = k - (A[i - 1] + A[i]);
            A[i] += d;
            cnt += d;
        }
    }
    
    cout << cnt << endl;
    for (int i : A) {
        cout << i << " ";
    }
    cout << endl;
    
    return 0;
}