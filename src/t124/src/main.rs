struct NumArray {
    tree: Vec<i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; n << 1];
        for i in n..n << 1 {
            tree[i] = nums[i - n];
        }
        for i in (0..n).rev() {
            tree[i] = tree[i << 1] + tree[i << 1 | 1]
        }
        NumArray { tree, n: n as i32 }
    }

    fn update(&mut self, mut index: i32, val: i32) {
        index += self.n;
        self.tree[index as usize] = val;
        while index > 0 {
            self.tree[index as usize >> 1] =
                self.tree[index as usize] + self.tree[index as usize ^ 1];
            index >>= 1;
        }
    }

    fn sum_range(&self, mut left: i32, mut right: i32) -> i32 {
        left += self.n;
        right += self.n;
        let mut ret = 0;
        while left <= right {
            if (left & 1) == 1 {
                ret += self.tree[left as usize];
                left += 1;
            }
            if (right & 1) == 0 {
                ret += self.tree[right as usize];
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        ret
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

fn main() {}
