#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

ll N, M;
int digit_hour;
int digit_min;
int digit;
int v[10];

int get_digit(int n) { // in base 7
    int cnt = 0;
    while (n) {
        n /= 7;
        cnt++;
    }
    return max(1, cnt);
}

bool is_legal() {
    ll h = 0;
    for (int i = 0; i < digit_hour; i++) {
        h = h * 7 + v[i];
    }
    if (h >= N) return false;

    ll m = 0;
    for (int i = digit_hour; i < digit; i++) {
        m = m * 7 + v[i];
    }
    if (m >= M) return false;

    return true;
}

ll ans;
bool used[10];
void dfs(int idx) {
    if (idx == digit) {
        if (is_legal())
            ans++;
        return;
    }

    for (int j = 0; j < 7; j++) {
        if (!used[j]) {
            v[idx] = j;
            used[j] = true;
            dfs(idx + 1);
            used[j] = false;
        }
    }
}

int main() {
    scanf("%lld %lld", &N, &M);

    digit_hour = get_digit(N - 1);
    digit_min = get_digit(M - 1);

    digit = digit_hour + digit_min;

    if (digit > 7) {
        puts("0");
        return 0;
    }

    fill(used, used + digit, false);
    ans = 0;
    dfs(0);

    printf("%lld\n", ans);

    return 0;
}
