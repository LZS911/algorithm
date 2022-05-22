#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

struct TreeNode
{
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
public:
  vector<int> getAllElements(TreeNode *root1, TreeNode *root2)
  {
    vector<int> a, b, c;
    auto dfs = [&](auto dfs, auto tag, auto &a)
    {
      if (!tag)
      {
        return;
      }
      dfs(dfs, tag->right, a);
      a.emplace_back(tag->val);
      dfs(dfs, tag->left, a);
    };
    dfs(dfs, root1, a);
    dfs(dfs, root2, b);
    while (!a.empty() || !b.empty())
    {
      if (!a.empty() && !b.empty())
      {
        if (a.back() > b.back())
        {
          c.emplace_back(b.back());
          b.pop_back();
        }
        else
        {
          c.emplace_back(a.back());
          a.pop_back();
        }
      }
      else if (!a.empty())
      {
        c.emplace_back(a.back());
        a.pop_back();
      }
      else
      {
        c.emplace_back(b.back());
        b.pop_back();
      }
    }
    return c;
  }
};
int main()
{
  Solution s;
}