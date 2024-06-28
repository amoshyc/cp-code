#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N; cin >> N;
    
    vector<bool> cnt(1000 + 1, false);
    for (int i = 0; i < N; i++) {
        int inp; cin >> inp;
        cnt[inp] = true;
    }
    
    for (int i = 1; i + 2 <= 1000; i++) {
        if (cnt[i] && cnt[i + 1] && cnt[i + 2]) {
            cout << "YES\n";
            return 0;
        }
    }
    
    cout << "NO\n";
    
    
    return 0;
}