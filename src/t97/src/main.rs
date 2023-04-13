impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let (n, mut i, mut j) = (arr.len(), arr.len() - 2, arr.len() - 1);
        while i < n && arr[i] <= arr[i + 1] {
            i -= 1;
        }
        if i >= n {
            return arr;
        }
        while arr[i] <= arr[j] || j < n && arr[j] == arr[j - 1] {
            j -= 1;
        }
        arr.swap(i, j);
        arr
    }
}
