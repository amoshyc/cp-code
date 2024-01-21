#include <algorithm>
#include <atcoder/dsu>
#include <iostream>
#include <map>
#include <tuple>
#include <vector>
using namespace std;

struct Town {
    int id, r, c;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    vector<Town> towns(N);
    for (int i = 0; i < N; i++) {
        int r, c;
        cin >> r >> c;
        --r;
        --c;
        towns[i] = {i, r, c};
    }

    sort(towns.begin(), towns.end(),
         [](auto town_a, auto town_b) { return town_a.r < town_b.r; });

    auto components = atcoder::dsu(N);
    auto vis = vector<bool>(N, false);
    int perm = N;

    int s = 0;
    while (s < N) {
        int t = s + 1;
        int min_col = towns[s].c;
        vis[towns[s].c] = true;
        while (perm >= 1 and vis[perm - 1]) {
            perm--;
        }
        while (t < N and perm != min_col) {
            min_col = min(min_col, towns[t].c);
            vis[towns[t].c] = true;
            while (perm >= 1 and vis[perm - 1]) {
                perm--;
            }
            components.merge(towns[s].id, towns[t].id);
            t++;
        }
        s = t;
    }

    for (int i = 0; i < N; i++) {
        cout << components.size(i) << endl;
    }

    return 0;
}