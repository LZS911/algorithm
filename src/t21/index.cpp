#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution
{
public:
  string convert(string s, int numRows)
  {
    int len = s.length();
    if (numRows == 1 || len <= numRows || len <= 2)
    {
      return s;
    }
    int flag = -1;
    int y = 0;
    vector<string> res(numRows);
    for (int i = 0; i < len; ++i)
    {
      char item = s[i];
      res[y] += item;
      if (y == 0 || y == numRows - 1)
      {
        flag = -flag;
      }
      y += flag;
    }
    for (int i = 1; i < numRows; ++i)
    {
      res[0] += res[i];
    }
    return res[0];
  }
};

int main()
{
  Solution s;
  string res1 = s.convert("PAYPALISHIRING", 3);
}