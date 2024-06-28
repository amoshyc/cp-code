#include <bits/stdc++.h>
using namespace std;
 
typedef long long ll;
typedef pair<int, int> pii;

const int max_n = 2000;
const int max_m = 2000;
int n, m;
ll a[max_n];
ll b[max_m];

ll ans;
pii swap1 = pii(-1, -1);
pii swap2 = pii(-1, -1);

ll Sa;
ll Sb;
map<ll, pii> mm;

void update(int i, int j, int k, int l) {
    ll nSa = Sa - a[i] - a[j] + b[k] + b[l];
    ll nSb = Sb - b[k] - b[l] + a[i] + a[j];
    ll new_ans = abs(nSa - nSb);

    if (new_ans < ans) {
        ans = new_ans;
        swap1 = pii(i, k);
        swap2 = pii(j, l);
    }
}

void solve() {
    Sa = accumulate(a, a + n, 0ll);
    Sb = accumulate(b, b + m, 0ll);

    // 0 swap
    ans = abs(Sa - Sb);

    // 1 swap
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            ll nSa = Sa - a[i] + b[j];
            ll nSb = Sb - b[j] + a[i];
            ll new_ans = abs(nSa - nSb);
            if (new_ans < ans) {
                ans = new_ans;
                swap1 = pii(i, j);
            }
        }
    }

    // 2 swaps
    for (int i = 0; i < n - 1; i++) {
        for (int j = i + 1; j < n; j++) {
            mm[Sb + 2 * (a[i] + a[j])] = pii(i, j);
        }
    }
    
    for (int k = 0; k < m - 1; k++) {
        for (int l = k + 1; l < m; l++) {
            ll val = Sa + 2 * (b[k] + b[l]);
            auto it = mm.lower_bound(val);
            
            if (it != mm.end())
                update((it->second).first, (it->second).second, k, l);
            if (it != mm.begin()) {
                it--;
                update((it->second).first, (it->second).second, k, l);
            }
        }
    }
}

int main() {
    scanf("%d", &n);
    for (int i = 0; i < n; i++)
        scanf("%lld", &a[i]);
    scanf("%d", &m);
    for (int i = 0; i < m; i++)
        scanf("%lld", &b[i]);

    solve();

    printf("%lld\n", ans);
    // printf("%I64d\n", ans);
    if (swap1.first == -1) {
        puts("0");
    }
    else if (swap2.first == -1) {
        puts("1");
        printf("%d %d\n", swap1.first + 1, swap1.second + 1);
    }
    else {
        puts("2");
        printf("%d %d\n", swap1.first + 1, swap1.second + 1);
        printf("%d %d\n", swap2.first + 1, swap2.second + 1);
    }

    return 0;
}