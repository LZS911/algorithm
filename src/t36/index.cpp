#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;
class Solution
{
public:
  int findTheWinner(int n, int k)
  {
    vector<int> visited(n, 0);
    int count = n, cur = 0;
    while (count != 1)
    {
      for (int i = 0; i < k - 1; i++)
      {
        cur++;
        cur = cur % n;
        while (visited[cur])
        {
          cur++;
          cur = cur % n;
        }
      }
      visited[cur] = 1;
      count--;
      cur++;
      cur = cur % n;
      while (visited[cur])
      {
        cur++;
        cur = cur % n;
      }
    }
    return cur + 1;
  }
};

int main()
{
  Solution s;
  int res1 = s.findTheWinner(6, 5);
  int res2 = s.findTheWinner(5, 2);
}