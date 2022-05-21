#include <iostream>
#include <vector>
#include <string>
#include <cmath>
using namespace std;

class Solution
{
public:
  int countPrimeSetBits(int left, int right)
  {
    int ans = 0;

    for (int i = left; i <= right; i++)
    {
      int cnt = 0;
      int x = i;
      while (x)
      {
        cnt++;
        x &= (x - 1);
      }
      if (cnt == 2 || cnt == 3 || cnt == 5 || cnt == 7 || cnt == 11 || cnt == 13 || cnt == 17 || cnt == 19 || cnt == 23)
        ans++;
    }

    return ans;
  }
};
int main()
{
  Solution s;
  s.countPrimeSetBits(1, 10);
  s.countPrimeSetBits(3, 10);
}