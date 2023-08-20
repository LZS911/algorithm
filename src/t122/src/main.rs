impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index = (m + n) as usize - 1;
        let mut i1 = m as usize;
        let mut i2 = n as usize;
        while i1 > 0 && i2 > 0 {
            //rust的这个下标是usize，容易溢出，好烦
            if nums1[i1 - 1] >= nums2[i2 - 1] {
                nums1[index] = nums1[i1 - 1];
                i1 -= 1;
            } else {
                nums1[index] = nums2[i2 - 1];
                i2 -= 1;
            }
            index -= 1;
        }
        while i2 > 0 {
            nums1[index] = nums2[i2 - 1];
            i2 -= 1;
            index -= 1;
        }
    }
}
