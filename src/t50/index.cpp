#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>
#include <numeric>

using namespace std;
class Solution
{
public:
  int oddCells(int m, int n, vector<vector<int>> &indices)
  {
    vector<int> rows(m), cols(n);
    for (int i = 0; i < indices.size(); ++i)
    {
      rows[indices[i][0]]++;
      cols[indices[i][1]]++;
    }
    int res = 0;
    for (int i = 0; i < m; ++i)
    {
      for (int j = 0; j < n; ++j)
      {
        if ((rows[i] + cols[j]) & 1)
        {
          res++;
        }
      }
    }
    return res;
  }
};
int main()
{
  Solution s;
  vector<vector<int>> p1 = {vector<int>{0, 1}, vector<int>{1, 1}};
  int res1 = s.oddCells(2, 3, p1);

  vector<vector<int>> p2 = {vector<int>{1, 1}, vector<int>{0, 0}};
  int res2 = s.oddCells(2, 2, p2);
}