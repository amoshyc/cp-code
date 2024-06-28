#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
#define sz(x) (int(x.size()))

const int MAX_N = 200000;
const int MAX_M = 200000;

struct Item {
    int id, val;
    bool operator < (const Item& it) const {
        if (val == it.val)
            return id < it.id;
        return val < it.val;
    }
};

int N, M;
set<Item> P;
int S[MAX_M];

int n_ada = 0;
int ada[MAX_M]; // i-th socket need ada[i] adapters
int n_match = 0;
int match[MAX_N]; // i-th pc <-> match[i]-th socket

void solve() {
    n_ada = 0;
    fill(ada, ada + M, 0);
    n_match = 0;
    fill(match, match + N, -1);

    for (int i = 0; i < 32; i++) {
        for (int j = 0; j < M; j++) {
            if (S[j] == 0) continue;

            auto lb = P.lower_bound(Item{-1, S[j]});
            if (lb != P.end() && lb->val == S[j]) {
                n_ada += i;
                ada[j] = i;
                n_match++;
                match[lb->id] = j;
                P.erase(lb);
                S[j] = 0;
            }
            else {
                S[j] = (S[j] + 1) / 2;
            }
        }
    }
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        Item it; it.id = i; scanf("%d", &it.val);
        P.insert(it);
    }
    for (int i = 0; i < M; i++) {
        scanf("%d", &S[i]);
    }

    solve();
    printf("%d %d\n", n_match, n_ada);
    for (int i = 0; i < M; i++) {
        printf("%d ", ada[i]);
    }
    puts("");
    for (int i = 0; i < N; i++) {
        printf("%d ", match[i] + 1);
    }
    puts("");

    return 0;
}
