#include <string>

using namespace std;

class Solution
{
public:
  bool rotateString(string s, string goal)
  {
    if (s.length() != goal.length())
    {
      return false;
    }
    s = s + s;
    if (s.find(goal) == string::npos)
    {
      return false;
    }
    return true;
  }
};

int main()
{
  Solution s;

  s.rotateString("abcde", "cdeab");
}