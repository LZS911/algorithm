#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution
{
public:
  long long subArrayRanges(vector<int> &nums)
  {
    long long res = 0;
    vector<int> path{};

    for (int i = 0; i < nums.size(); ++i)
    {
      int min = nums[i];
      int max = nums[i];
      for (int j = i; j < nums.size(); ++j)
      {

        if (min > nums[j])
        {
          min = nums[j];
        }
        if (max < nums[j])
        {
          max = nums[j];
        }

        res += max - min;
      }
    }
    return res;
  }
};
int main()
{
  Solution s;
  vector<int> res{4, -2, -3, 4, 1};
  long long res1 = s.subArrayRanges(res);
}