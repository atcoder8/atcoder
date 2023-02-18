#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
using ll = long long;

int main() {
    ll n, m;
    cin >> n >> m;
    vector<ll> aa(m);
    for (int i = 0; i < m; i++) cin >> aa[i];

    dsu uf(n);
    for (auto&& a : aa) {
        uf.merge(a - 1, a);
    }

    auto groups = uf.groups();
    sort(groups.begin(), groups.end());

    for (auto&& group : groups) {
        sort(group.begin(), group.end(), greater<ll>());

        for (auto&& node : group) {
            cout << node + 1 << " ";
        }
    }
    cout << "\n";
}
