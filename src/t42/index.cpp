#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <queue>

using namespace std;

class Solution
{
public:
  double largestTriangleArea(vector<vector<int>> &points)
  {
    double ans = 0;
    for (vector<int> &p1 : points)
    {
      int x1 = p1[0], y1 = p1[1];
      for (vector<int> &p2 : points)
      {
        int x2 = p2[0], y2 = p2[1];
        for (vector<int> &p3 : points)
        {
          int x3 = p3[0], y3 = p3[1];
          ans = max(ans, 0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)));
        }
      }
    }
    return ans;
  }
};

int main()
{
  Solution s;
  vector<vector<int>> v{{0, 0}, {0, 1}, {1, 0}, {0, 2}, {2, 0}};
  double res = s.largestTriangleArea(v);
}