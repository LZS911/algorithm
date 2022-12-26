export function minElements(nums: number[], limit: number, goal: number): number {
  const sum = nums.reduce((acc, cur) => acc + cur, 0);

  const d = Math.abs(sum - goal);

  return Math.floor((d + limit - 1) / limit);
}

console.log(minElements([1, -1, 1], 3, -4));
