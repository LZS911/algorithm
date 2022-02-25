#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution
{
public:
  vector<vector<int>> combinationSum3(int k, int n)
  {
    res = {};
    dfs(vector<int>{}, k, n, 1);
    return res;
  }
  vector<vector<int>> res;

private:
  void dfs(vector<int> path, int sum, int residue, int startIndex)
  {
    if (path.size() == sum && residue == 0)
    {
      res.push_back(path);
      return;
    }

    for (int i = startIndex; i <= 9; ++i)
    {
      path.push_back(i);
      dfs(path, sum, residue - i, i + 1);
      path.pop_back();
    }
  }
};

int main()
{
  Solution s;
  vector<vector<int>> res1 = s.combinationSum3(3, 7);
  vector<vector<int>> res2 = s.combinationSum3(3, 9);
  vector<vector<int>> res3 = s.combinationSum3(4, 10);
}