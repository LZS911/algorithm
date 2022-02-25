#include <iostream>
#include <vector>
#include <string>
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
  vector<string> binaryTreePaths(TreeNode *root)
  {
    if (!root)
    {
      return vector<string>{};
    }
    dfs(root, to_string(root->val));
    return res;
  }

private:
  vector<string> res;
  void dfs(TreeNode *root, string val)
  {
    if (!root)
    {
      return;
    }

    if (!root->left && !root->right)
    {
      res.push_back(val);
    }

    if (root->left)
    {
      dfs(root->left, val + "->" + to_string(root->left->val));
    }
    if (root->right)
    {
      dfs(root->right, val + "->" + to_string(root->right->val));
    }
  }
};