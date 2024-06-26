#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const ll MAX_NUM = 2 * 1000000;
bool is_prime[MAX_NUM];
void init_primes() {
    fill(is_prime, is_prime + MAX_NUM, true);
    is_prime[0] = is_prime[1] = false;
    for (ll i = 2; i < MAX_NUM; i++) {
        if (is_prime[i]) {
            for (ll j = i * i; j < MAX_NUM; j += i)
                is_prime[j] = false;
        }
    }
}

void solve(vector<int>& v) {
    vector<int> odd, even;
    int cnt1 = 0;
    for (int i : v) {
        if (i & 1) odd.push_back(i);
        else even.push_back(i);

        if (i == 1) cnt1++;
    }

    // 2+ even number is impossible
    // 2+ odd number is impossible

    // Case 1: 1+, one even, sum is prime
    if (cnt1 >= 1) {
        for (int e : even) {
            if (is_prime[e + 1]) {
                cout << cnt1 + 1 << "\n";
                for (int i = 0; i < cnt1; i++)
                    cout << 1 << " ";
                cout << e << "\n";
                return;
            }
        }
    }

    // Case 2: all 1
    if (cnt1 >= 2) {
        cout << cnt1 << "\n";
        for (int i = 0; i < cnt1; i++)
            cout << 1 << " ";
        cout << "\n";
        return;
    }

    // Case 3: one odd, one even, sum is prime
    for (int o : odd) {
        for (int e : even) {
            if (is_prime[o + e]) {
                cout << 2 << "\n";
                cout << o << " " << e << " " << "\n";
                return;
            }
        }
    }

    // Case 4: one number
    cout << 1 << "\n";
    cout << v[0] << "\n";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    init_primes();

    int N; cin >> N;
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++)
        cin >> data[i];

    solve(data);

    return 0;
}
