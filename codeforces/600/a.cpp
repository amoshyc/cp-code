#include <bits/stdc++.h>
using namespace std;

bool is_str_digit(string s) {
    if (s.length() == 0) return false;

    for (char c : s)
        if (c < '0' || c > '9')
            return false;
    return true;
}

int main() {
    ios_base::sync_with_stdio(false);

    string inp;
    cin >> inp;

    replace(inp.begin(), inp.end(), ',', ';');

    vector<string> a;
    vector<string> b;

    inp += ";";
    istringstream iss(inp);
    string token;
    while (getline(iss, token, ';')) {
        // cout << "|" << token << "|" << endl;
        if ((is_str_digit(token) && token[0] != '0') || token == "0")
            a.push_back(token);
        else
            b.push_back(token);
    }

    if (a.empty()) {
        cout << "-\n";
    }
    else {
        cout << "\"";
        for (size_t i = 0; i < a.size(); i++) {
            if (i != 0) cout << ",";
            cout << a[i];
        }
        cout << "\"\n";
    }

    if (b.empty()) {
        cout << "-\n";
    }
    else {
        cout << "\"";
        for (size_t i = 0; i < b.size(); i++) {
            if (i != 0) cout << ",";
            cout << b[i];
        }
        cout << "\"\n";
    }

    return 0;
}
