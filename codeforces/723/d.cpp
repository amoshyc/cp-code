#include <bits/stdc++.h>
using namespace std;

const int dr[4] = {0, 0, -1, +1};
const int dc[4] = {+1, -1, 0, 0};

struct Lake {
	int r, c, sz;
};

bool cmp_by_sz(const Lake& a, const Lake& b) {
	return a.sz < b.sz;
}

int N, M, K;
string A[50];
bool vis[50][50];
vector<Lake> lakes;

bool isLake;
int sz;
void dfs1(int r, int c) {
	vis[r][c] = true;
	sz++;

	for (int i = 0; i < 4; i++) {
		int nr = r + dr[i];
		int nc = c + dc[i];
		if (nr < 0 || nr >= N || nc < 0 || nc >= M) {
			isLake = false;
			continue;
		}
		if (A[nr][nc] != '.' || vis[nr][nc])
			continue;

		dfs1(nr, nc);
	}
}

void dfs2(int r, int c) {
	A[r][c] = '*';
	for (int i = 0; i < 4; i++) {
		int nr = r + dr[i];
		int nc = c + dc[i];
		if (A[nr][nc] != '.') continue;
		dfs2(nr, nc);
	}
}

int main() {
	ios::sync_with_stdio(false);
	cin.tie(0);

	cin >> N >> M >> K;
	for (int r = 0; r < N; r++)
		cin >> A[r];

	for (int r = 0; r < N; r++) {
		for (int c = 0; c < M; c++) {
			if (A[r][c] == '.' && !vis[r][c]) {
				isLake = true;
				sz = 0;
				dfs1(r, c);

				if (isLake) {
					lakes.push_back((Lake) {r, c, sz});
				}
			}
		}
	}

	// cout << "........" << endl;

	sort(lakes.begin(), lakes.end(), cmp_by_sz);
	reverse(lakes.begin(), lakes.end());

	// for (auto l : lakes) {
	// 	cout << l.r << " " << l.c << " " << l.sz << endl;
	// }
	// cout << endl;

	int ans = 0;
	while (int(lakes.size()) > K) {
		Lake lake = lakes.back(); lakes.pop_back();
		ans += lake.sz;
		dfs2(lake.r, lake.c);
	}

	cout << ans << endl;
	for (int r = 0; r < N; r++) {
		cout << A[r] << endl;
	}

	return 0;
}
