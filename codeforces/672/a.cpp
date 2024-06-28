#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    int N; cin >> N;
    string s = "";
    int idx = 1;
    while (s.length() < N) {
        stringstream ss; ss << idx;
        string temp; ss >> temp;
        s += temp;
        idx++;
    }

    cout << s[N - 1] << endl;

    return 0;
}
