#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))

const int MAX_N = 2000;
int N, M;
vector<int> a;
vector<int> u;
vector<int> cnt[MAX_N + 1];

int main() {
	scanf("%d %d", &N, &M);

	a.resize(N);
	for (int i = 0; i < N; i++) {
		scanf("%d", &a[i]);

		if (a[i] <= M)
			cnt[a[i]].push_back(i);
		else
			u.push_back(i);
	}

	int num = N / M;
	for (int i = 1; i <= M; i++) {
		while (sz(cnt[i]) > num) {
			u.push_back(cnt[i].back());
			cnt[i].pop_back();
		}
	}

	int ans = 0;
	for (int i = 1; i <= M; i++) {
		while (sz(cnt[i]) < num) {
			int idx = u.back(); u.pop_back();
			cnt[i].push_back(idx);
			a[idx] = i;
			ans++;
		}
	}

	printf("%d %d\n", num, ans);
	for (int i : a) {
		printf("%d ", i);
	}
	puts("");

	return 0;
}
