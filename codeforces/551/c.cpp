#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;
#define sz(x) (int(x.size()))
#define st first
#define nd second

const int MAX_N = 100000;
int N, M;
ll num[MAX_N];

ll ceildiv(ll a, ll b) {
    return (a + b - 1) / b;
}

// 在 t 秒內，M 個人可否搬完所有石頭
// 即在 t 秒內，搬完所有石頭所需的人數是否 <= M
bool C(ll t) {
    ll need = 0;
    ll pass = 0; // 記錄（搬完右邊後）殘餘人力，類似 poj 2431 加油問題
    // 從右至左迭代才沒有後效性，因為人只能向右走
    for (int i = N - 1; i >= 0; i--) {
        if (num[i] == 0) continue;
        if (pass >= num[i]) { // 優先使用殘餘人力
            pass -= num[i];
            continue;
        }

        ll rest = num[i] - pass; // 此堆剩下的石頭數
        ll t_walk = i + 1; // 走到此堆所需的時間
        if (t_walk >= t) { // 已超過 t 秒...
            return false;
        }
        ll n_take = t - t_walk; // 一個人在 t 秒，可以搬走幾個第 i 堆的石頭
        ll need_i = ceildiv(rest, n_take); // 搬完此堆石頭所需的人數

        need += need_i;
        pass = need_i * n_take - rest; // 更新殘餘人力
    }
    return need <= M;
}

ll solve() {
    // binary search on time
    // 0 0 0 1 1 1
    ll lb = 0, ub = 1e17;
    while (ub - lb > 1) {
        ll m = (lb + ub) / 2;
        if (C(m)) ub = m;
        else lb = m;
    }
    return ub;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        scanf("%lld", &num[i]);
    }

    printf("%lld\n", solve());

    return 0;
}
