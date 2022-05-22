#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;

class Solution
{
public:
  bool oneEditAway(string f, string s)
  {
    int n = f.size(), m = s.size();
    if (f == s || n == 0 && m == 1 || n == 1 && m == 0)
      return true;
    if (abs(n - m) <= 1)
    {
      int l = 0, r = n - 1;
      int x = 0, y = m - 1;
      while (l < n && x < m && f[l] == s[x])
      {
        l++, x++;
      }
      while (r >= 0 && y >= 0 && f[r] == s[y])
      {
        r--, y--;
      }
      if (n >= m)
        return l == r;
      else
        return x == y;
    }
    return false;
  }
};

int main()
{
  Solution s;
  bool res1 = s.oneEditAway("pale", "ple");
  bool res2 = s.oneEditAway("pales", "pal");
}