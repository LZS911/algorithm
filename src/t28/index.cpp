#include <iostream>
#include <vector>
#include <string>
#include <cmath>
#include <map>
#include <queue>
using namespace std;

class Solution
{
public:
  vector<int> findMinHeightTrees(int n, vector<vector<int>> &edges)
  {
    if (n == 1)
      return {0};
    vector<int> degree(n);
    map<int, vector<int>> m;
    vector<int> res;
    for (int i = 0; i < edges.size(); i++)
    {
      int u = edges[i][0];
      int v = edges[i][1];
      degree[u]++;
      degree[v]++;
      m[u].push_back(v);
      m[v].push_back(u);
    }
    queue<int> q;
    for (int i = 0; i < n; i++)
      if (degree[i] == 1)
        q.push(i);
    while (!q.empty())
    {
      res.clear();
      int sz = q.size();
      for (int i = 0; i < sz; i++)
      {
        int t = q.front();
        q.pop();
        res.push_back(t);
        degree[t]--;
        for (auto j : m[t])
        {
          degree[j]--;
          if (degree[j] == 1)
            q.push(j);
        }
      }
    }
    return res;
  }
};
int main()
{
  Solution s;
  vector<int> v1{1, 0};
  vector<int> v2{1, 2};
  vector<int> v3{1, 3};
  vector<vector<int>> v{v1, v2, v3};
  s.findMinHeightTrees(4, v);
}