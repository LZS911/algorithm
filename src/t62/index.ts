export function partitionDisjoint(nums: number[]): number {
  let max = nums[0];
  let leftMax = max;
  let index = 0;

  for (let i = 1; i < nums.length; ++i) {
    const num = nums[i];
    if (leftMax > num) {
      leftMax = max;
      index = i;
    } else {
      max = Math.max(max, num);
    }
  }

  return index + 1;
}
