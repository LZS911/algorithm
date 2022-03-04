#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution
{
public:
  int atoi(char s)
  {
    return s - 48;
  }
  int addDigits(int num)
  {
    if (0 <= num && num < 10)
    {
      return num;
    }

    string numStr = to_string(num);
    int sum = 0;
    for (int i = 0; i < numStr.length(); ++i)
    {
      sum += atoi(numStr[i]);
    }
    return addDigits(sum);
  }
};

int main()
{
  Solution s;
  int res = s.addDigits(102);
}