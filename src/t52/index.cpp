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
  vector<int> shuffle(vector<int> &nums, int n)
  {
    vector<int> res(2 * n);

    for (int i = 0; i < n; ++i)
    {
      res[i * 2] = nums[i];
      res[i * 2 + 1] = nums[i + n];
    }

    return res;
  }
};

int main()
{
  Solution s;
  vector<int> res1;
  vector<int> p1{2, 5, 1, 3, 4, 7};
  res1 = s.shuffle(p1, 3);
}