#include <algorithm>
#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

using ll = long long;

// A template function mimics python's join
template <typename T>
string join(const vector<T> &arr, const string &seperator = " ") {
    ostringstream oss;
    for (size_t i = 0; i < arr.size(); i++) {
        if (i != 0) {
            oss << seperator;
        }
        oss << arr[i];
    }
    return oss.str();
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    vector<ll> A(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    vector<ll> S = A;
    sort(S.begin(), S.end());

    vector<ll> suff = S;
    for (int i = N - 2; i >= 0; i--) {
        suff[i] = suff[i + 1] + S[i];
    }

    vector<ll> ans(N, 0);
    for (int i = 0; i < N; i++) {
        // check(m) = is S[m] bigger than A[i]
        // The check function forms a ...0 0 0 1 1 1... distribution over the
        // indices of S. In this problem, we want to know where the first 1 is.
        // We implement check function using c++11 lambda:
        auto check = [&](int m) { return S[m] > A[i]; };

        // We find the partition point (the last 0 and the first 1) using binary
        // search.

        // Note there are 2 cases binary search won't work:
        // Case 1: The distribution is all 0: 0 0 0 ... 0 0 0
        // Case 2: The distribution is all 1: 1 1 1 ... 1 1 1
        // In this problem,
        // Case 1 occurs when A[i] is the maximum element of A
        // Case 2 will not occur.

        // Case 1
        if (check(N - 1) == false) { // A[i] is the maximum value of A
            ans[i] = 0;
            continue;
        }

        // Binary Search
        // We are finding the partition point.
        // 1. We set lower bound lb to the minimum possible index.
        // 2. We set upper bound ub to the maximum possible index.
        // 3. We examine the middle point m and narrow down lb or ub
        // accordingly.
        // 4. Repeat Step 3 until lb is adjacent to ub
        // 5. lb stops at the last 0, ub stops at the first 1

        // if you don't know which should you update, lb or ub?
        // Remember for a ...0 0 0 1 1 1... distribution, 
        // there are 2 invariants:
        // 1. check(lb) is always 0.
        // 2. check(ub) is always 1.
        // Therefore 
        // if check(m) is 0, update lb,
        // if check(m) is 1, update ub.

        int lb = 0, ub = N - 1;
        while (ub - lb > 1) {
            int m = (lb + ub) / 2;
            if (check(m)) {
                ub = m;
            } else {
                lb = m;
            }
        }

        ans[i] = suff[ub];
    }

    cout << join(ans, " ") << endl;

    return 0;
}