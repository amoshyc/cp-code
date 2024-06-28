#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))
typedef long long ll;
inline int getint() { int inp; scanf(" %d", &inp); return inp; }
inline int getll() { ll inp; scanf(" %lld", &inp); return inp; }

const int MAX_N = 200000 + 10;
int N;
int last[MAX_N];

int main() {
    N = getint();

    vector<int> cs;
    for (int i = 0; i < N; i++) {
        int id = getint();
        cs.push_back(id);
        last[id] = i;
    }

    sort(cs.begin(), cs.end());
    int NN = unique(cs.begin(), cs.end()) - cs.begin();
    cs.resize(NN);

    int ans = cs[0];
    for (int i = 1; i < sz(cs); i++) {
        int id = cs[i];
        if (last[id] < last[ans]) {
            ans = id;
        }
    }

    printf("%d\n", ans);    

    return 0;
}