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
  vector<int> findDiagonalOrder(vector<vector<int>> &mat)
  {
    vector<int> result;
    int all = mat.size() * mat[0].size();
    int x = 0, y = 0;
    for (int i = 0; i < all; i++)
    {
      result.push_back(mat[x][y]);
      if ((x + y) % 2 == 0)
      {
        if (y == mat[0].size() - 1)
          x++;
        else if (x == 0)
          y++;
        else
          x--, y++;
      }
      else
      {
        if (x == mat.size() - 1)
          y++;
        else if (y == 0)
          x++;
        else
          x++, y--;
      }
    }
    return result;
  }
};

int main()
{
  Solution s;
  vector<int> v1{1, 2, 3};
  vector<int> v2{4, 5, 6};
  vector<int> v3{7, 8, 9};

  vector<vector<int>> req = {v1, v2, v3};
  vector<int> res = s.findDiagonalOrder(req);
}