#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

class Solution
{
public:
  vector<vector<int>> res;
  vector<vector<int>> subsetsWithDup(vector<int> &nums)
  {
    sort(nums.begin(), nums.end());
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
      if (i > index && nums[i] == nums[i - 1])
      {
        continue;
      }
      path.push_back(nums[i]);
      dfs(nums, path, i + 1);
      path.pop_back();
    }
  }
};

int main()
{
  Solution s;

  vector<int> list1{1, 2, 2};
  vector<vector<int>> res1 = s.subsetsWithDup(list1);

  vector<int> list2{0};
  vector<vector<int>> res2 = s.subsetsWithDup(list2);
}