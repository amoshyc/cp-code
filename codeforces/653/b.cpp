#include <bits/stdc++.h>
using namespace std;

int N, M;
int cnt = 0;
vector<string> T[6];

void dfs(string s) {
    if (int(s.length()) == N) {
        cnt++;
        return;
    }
    
    char last = s.back();
    s.pop_back();
    
    for (string t : T[last - 'a']) {
        dfs(s + t);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    cin >> N >> M;
    while (M--) {
        string s, t; cin >> s >> t;
        reverse(s.begin(), s.end());
        T[t[0] - 'a'].push_back(s);
    }
    
    dfs("a");
    
    cout << cnt << "\n";
    
    return 0;
}