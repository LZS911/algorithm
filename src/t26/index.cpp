#include <iostream>
#include <vector>
#include <string>
#include <cmath>
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
  bool hasPathSum(TreeNode *root, int targetSum)
  {
    if (!root || targetSum < 0)
    {
      return false;
    }
    if (root->left == nullptr && root->right == nullptr)
    {
      return targetSum - root->val == 0;
    }
    return hasPathSum(root->left, targetSum - root->val) || hasPathSum(root->right, targetSum - root->val);
  }
};
int main()
{
  Solution s;
}