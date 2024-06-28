#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

template <class T>
struct BIT {
    int R;
    vector<T> v;
    BIT();
    BIT(int r);
    T sum(int idx);
    void add(int idx, T x);
};

int N;
vector<int> A;
vector<int> L;
vector<int> R;
map<int, int> cntL, cntR;

BIT<ll> bit;

int main() {
    scanf("%d", &N);

    A = vector<int>(N, 0);
    L = vector<int>(N, 0);
    R = vector<int>(N, 0);
    bit = BIT<ll>(N);

    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }

    for (int i = 0; i < N; i++) {
        L[i] = ++cntL[A[i]];
    }

    for (int i = N - 1; i >= 0; i--) {
        R[i] = ++cntR[A[i]];
    }

    ll ans = 0;
    for(int j = N - 1; j >= 0; j--) {
		ans += bit.sum(L[j] - 1);
		bit.add(R[j], 1);
	}

    printf("%lld\n", ans);

    return 0;
}

template <class T>
BIT<T>::BIT(){
    ;
}

template <class T>
BIT<T>::BIT(int r): R(r) {
    v = vector<T>(R + 1, 0);
}

template <class T>
T BIT<T>::sum(int idx) {
    T s = 0;
    for (int i = idx; i > 0; i -= (i & -i))
        s += v[i];
    return s;
}

template <class T>
void BIT<T>::add(int idx, T x) {
    for (int i = idx; i <= N; i += (i & -i))
        v[i] += x;
}
