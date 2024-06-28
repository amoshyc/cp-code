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
    int N; cin >> N;
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        cin >> data[i];

    int cnt = 0;
    vector<int> ans;

    ans.push_back(data[0]);
    for (int i = 1; i < N; i++) {
        if (gcd(data[i], data[i - 1]) != 1) {
            cnt++;
            ans.push_back(1);
        }
        ans.push_back(data[i]);
    }

    cout << cnt << "\n";
    for (int i : ans)
        cout << i << " ";
    cout << endl;

    return 0;
}
