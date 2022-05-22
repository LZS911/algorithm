#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;

class Solution
{
public:
  int minDeletionSize(vector<string> &strs)
  {
    int n = strs.size(), m = strs[0].size(), del = 0;
    for (int j = 0; j < m; j++)
    {
      char c = strs[0][j];
      bool flag = true;
      for (int i = 1; i < n; i++)
        if (c <= strs[i][j])
        {
          c = strs[i][j];
        }
        else
        {
          flag = false;
          break;
        }
      if (!flag)
        del++;
    }
    return del;
  }
};

int main()
{
  Solution s;
  vector<string> v1{"cba", "daf", "ghi"};
  vector<string> v2{"a", "b"};
  vector<string> v3{"zyx", "wvu", "tsr"};
  int res1 = s.minDeletionSize(v1);
  int res2 = s.minDeletionSize(v2);
  int res3 = s.minDeletionSize(v3);
}