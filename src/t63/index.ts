export function sumSubarrayMins(arr: number[]): number {
  let value = 0;

  for (let i = 0; i < arr.length; ++i) {
    let min = arr[i];
    for (let j = i; j < arr.length; ++j) {
      min = Math.min(arr[j], min);
      value += min;
    }
  }

  return value % (Math.pow(10, 9) + 7);
}

console.log(sumSubarrayMins([3, 1, 2, 4]));
