#include <bits/stdc++.h>
using namespace std;

const int MAX_K = 100000;
int N, K;
vector<int> A;
int cnt[MAX_K + 1];

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++) {
        int inp; scanf("%d", &inp);

        // discard those repeated number
        if (A.empty() || inp != A.back())
            A.push_back(inp);
    }

    // for (int i : A) {
    //     printf("%d ", i);
    // }
    // puts("");

    const int len = A.size();
    for (size_t i = 0; i < len; i++) {
        // number of stress decrease if removing A[i]
        int num = 0;
        if (i == 0 || i == len - 1) { // first item
            num = 1;
        }
        else if (A[i - 1] != A[i + 1]) {
            num = 1;
        }
        else { // if (A[i - 1] == A[i + 1])
            num = 2;
        }

        // printf("A[%d] = %d, %d\n", i, A[i], num);

        cnt[A[i]] += num;
    }

    // for (int i = 1; i <= K; i++) {
    //     printf("%d: %d\n", i, cnt[i]);
    // }

    auto it = max_element(cnt + 1, cnt + K + 1);
    int ans = it - (cnt + 1) + 1;
    printf("%d\n", ans);

    return 0;
}
