#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Solution
{
public:
  vector<vector<int>> res;
  vector<vector<int>> subsets(vector<int> &nums)
  {
    res = {};
    vector<int> p;
    dfs(nums, p, 0);
    return res;
  }

private:
  void dfs(vector<int> &nums, vector<int> &path, int index)
  {
    if (path.size() == nums.size())
    {
      res.push_back(path);
      return;
    }
    res.push_back(path);

    for (int i = index; i < nums.size(); ++i)
    {
      path.push_back(nums[i]);
      dfs(nums, path, i + 1);
      path.pop_back();
    }
  }
};

int main()
{
  Solution s;

  vector<int> list1{1, 2, 3};
  vector<vector<int>> res1 = s.subsets(list1);

  vector<int> list2{0};
  vector<vector<int>> res2 = s.subsets(list2);
}