#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

class Solution
{
public:
  vector<int> missingRolls(vector<int> &rolls, const int mean, const int n)
  {
    vector<int> res(n);
    int sum = 0;
    for (int i = 0; i < rolls.size(); i++)
    {
      sum += rolls[i];
    }
    sum = (rolls.size() + n) * mean - sum;
    int avg = sum / n;
    if (sum * 1.0 / n < 1.0 || sum * 1.0 / n > 6.0)
    {
      return vector<int>(0);
    }
    for (int i = 0; i < res.size(); i++)
    {
      res[i] = avg;
    }
    for (int i = 0; i < sum - avg * n; i++)
    {
      res[i] += 1;
    }
    return res;
  }
};

int main()
{
  Solution s;
  vector<int> p1{3, 2, 4, 3};
  vector<int> res1 = s.missingRolls(p1, 4, 2);
}