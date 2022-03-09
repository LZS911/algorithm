#include <iostream>
#include <vector>
#include <string>
#include <cmath>
using namespace std;

class Solution
{
public:
  string convertToBase7(int num)
  {
    if (num == 0)
    {
      return "0";
    }
    int remainder = 0;
    int res = abs(num);
    string converStr = "";
    while (res > 0)
    {
      remainder = res % 7;
      res = res / 7;
      converStr = to_string(remainder) + converStr;
    }
    return num < 0 ? "-" + converStr : converStr;
  }
};
int main()
{
  Solution s;
  string res1 = s.convertToBase7(100);
}