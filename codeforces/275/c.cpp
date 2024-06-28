#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int N, K;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> K;

    vector<int> A(N);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    sort(A.begin(), A.end());

    set<int> ans;
    for (int i : A) {
        if (i % K != 0) ans.insert(i);
        else {
            if (ans.find(i / K) == ans.end())
                ans.insert(i);
        }
    }

    cout << ans.size() << endl;

    return 0;
}
