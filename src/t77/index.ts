export function minOperations(nums: number[]): number {
  let ans = 0;
  let max = 0;

  for (const val of nums) {
    console.log(val);
    ans += Math.max(0, max + 1 - val);
    max = Math.max(max + 1, val);
  }
  return ans;
}
console.log(minOperations([1, 1, 1]));
