#include <bits/stdc++.h>
using namespace std;

const int MAX_N = 18;
int N;
double p[MAX_N][MAX_N];
double dp[(1 << MAX_N)][MAX_N];

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%lf", &p[i][j]);
        }
    }

    // Ivan 應為最後一個上擂臺

    // dp[S][i] =
    // 還沒被打敗（含擂主）的人所形成的集合為 S，現在擂主是 i，Ivan 最後獲勝的機率

    // 剩下 Ivan 自己: dp[{0}][0] = 1
    // 考慮下一個上擂臺打的人 j，p[i][j] 的機率 i 贏，p[j][i] 的機率 j 贏：
    // 前者狀態轉移為 dp[S \ {j}][i]
    // 後者狀態轉移為 dp[S \ {i}][j]
    // dp[S][i] = max([
    //      dp[S \ {j}][i] * p[i][j] + dp[S \ {i}][j] * p[j][i]
    // ])

    // 最後枚舉一開始的擂主，看是誰時 Ivan 的勝率最大

    dp[(1 << 0)][0] = 1;
    for (int s = 2; s < (1 << N); s++) {
        for (int i = 0; i < N; i++) {
            if (!(s & (1 << i))) // 如果 i 不在 s 中，即 i 已被打敗，無須枚舉
                continue;

            dp[s][i] = 0;
            for (int j = 0; j < N; j++) {
                if (!(s & (1 << j))) // 如果 j 不在 s 中，即 j 已被打敗，無須枚舉
                    continue;

                dp[s][i] = max(dp[s][i],
                    dp[s ^ (1 << j)][i] * p[i][j] +
                    dp[s ^ (1 << i)][j] * p[j][i]);
            }
        }
    }

    double ans = 0.0;
    for (int i = 0; i < N; i++)
        ans = max(ans, dp[(1 << N) - 1][i]);
    printf("%.10f\n", ans);

    return 0;
}
