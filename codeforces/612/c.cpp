#include <bits/stdc++.h>
using namespace std;

int main() {
    string s; cin >> s;
    int len = s.length();
    if (len % 2 == 1) {
        cout << "Impossible\n";
        return 0;
    }

    vector<int> data(len);
    for (int i = 0; i < len; i++) {
        if (s[i] == '[')
            data[i] = -4;
        if (s[i] == '(')
            data[i] = -3;
        if (s[i] == '<')
            data[i] = -2;
        if (s[i] == '{')
            data[i] = -1;
        if (s[i] == ']')
            data[i] = 4;
        if (s[i] == ')')
            data[i] = 3;
        if (s[i] == '>')
            data[i] = 2;
        if (s[i] == '}')
            data[i] = 1;
    }

    int cnt = 0;
    stack<int> st;
    for (int i = 0; i < len; i++) {
        if (st.empty() || data[i] * st.top() > 0) {
            st.push(data[i]);
            continue;
        }

        if (data[i] > 0 && st.top() < 0 && abs(data[i]) == abs(st.top())) {
            st.pop();
            continue;
        }

        if (data[i] > 0 && st.top() < 0 && abs(data[i]) != abs(st.top())) {
            cnt++;
            st.pop();
            continue;
        }

        st.push(data[i]);
    }

    if (!st.empty())
        cout << "Impossible\n";
    else
        cout << cnt << "\n";

    return 0;
}
