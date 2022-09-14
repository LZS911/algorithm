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
  double trimMean(vector<int> &arr)
  {
    int len = arr.size();
    sort(arr.begin(), arr.end());
    int partialSum = accumulate(arr.begin() + len / 20, arr.begin() + (19 * len / 20), 0);
    return partialSum / (len * 0.9);
  }
};

int main()
{
  Solution s;
  vector<int> arr1{1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3};
  double res1 = s.trimMean(arr1);
}