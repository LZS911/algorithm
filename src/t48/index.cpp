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
  string defangIPaddr(string address)
  {
    string res;
    for (char &c : address)
    {
      if (c == '.')
      {
        res += "[.]";
      }
      else
      {
        res += c;
      }
    }
    return res;
  }
};
int main()
{
  Solution s;
  string res1 = s.defangIPaddr("1.1.1.1");
  string res2 = s.defangIPaddr("255.100.50.0");
}