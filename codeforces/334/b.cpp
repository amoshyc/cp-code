#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii;
#define st first
#define nd second

set<pii> pts;
vector<int> xs;
vector<int> ys;

bool solve() {
    if (pts.size() != 8)
        return false;

    sort(xs.begin(), xs.end());
    sort(ys.begin(), ys.end());   

    xs.resize(unique(xs.begin(), xs.end()) - xs.begin());
    ys.resize(unique(ys.begin(), ys.end()) - ys.begin());

    if (xs.size() != 3 || ys.size() != 3)
        return false;
    
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (i == 1 && j == 1) 
                continue;
            if (pts.find(pii(xs[i], ys[j])) == pts.end())
                return false;
        }
    }

    return true;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    for (int i = 0; i < 8; i++) {
        int x, y;
        cin >> x >> y;
        pts.insert(pii(x, y));
        xs.push_back(x);
        ys.push_back(y);
    }

    cout << ((solve()) ? "respectable" : "ugly") << endl;    

    return 0;
}
